use axum::extract::ws::{Message, WebSocket};
use eyre::eyre;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

pub struct JsonClient<R, S> {
    socket: Option<WebSocket>,
    _recv: PhantomData<R>,
    _send: PhantomData<S>,
}

impl<R: DeserializeOwned, S: Serialize> JsonClient<R, S> {
    pub fn new(socket: WebSocket) -> Self {
        Self {
            socket: Some(socket),
            _recv: Default::default(),
            _send: Default::default(),
        }
    }

    pub async fn recv(&mut self) -> eyre::Result<Option<R>> {
        let Some(socket) = self.socket.as_mut() else { return Ok(None) };

        let msg = match socket.recv().await {
            Some(msg) => msg?,
            None => return Ok(None),
        };

        match msg {
            Message::Text(doc) => Ok(Some(serde_json::from_str(&doc)?)),
            Message::Close(cf) => {
                let _ = socket.send(Message::Close(cf)).await;
                Ok(None)
            }
            _ => Err(eyre!("Invalid message type received")),
        }
    }
}
