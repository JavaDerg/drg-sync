use std::marker::PhantomData;
use axum::Error;
use axum::extract::ws::{Message, WebSocket};
use eyre::eyre;
use serde::{Deserialize, Serialize};

pub struct JsonClient<R, S> {
    socket: Option<WebSocket>,
    _recv: PhantomData<R>,
    _send: PhantomData<S>,
}

impl<R: Deserialize, S: Serialize> JsonClient<R, S> {
    pub async fn recv(&mut self) -> eyre::Result<Option<S>> {
        let Some(socket) = self.socket.as_mut() else { return Ok(None) };

        let msg = match socket.recv().await {
            Some(msg) => msg?,
            None => Ok(None),
        };

        match msg {
            Message::Text(doc) => Ok(Some(
                serde_json::from_str(&doc)?
            )),
            Message::Close(cf) => {
                let _ = socket.send(Message::Close(cf)).await;
                Ok(None)
            }
            _ => Err(eyre!("Invalid message type received")),
        }
    }
}
