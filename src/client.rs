use crate::room::{RoomHandle, RoomUpdate};
use axum::extract::ws::WebSocket;
use tokio::sync::broadcast;
use uuid::Uuid;
use crate::client::json::JsonClient;

mod json;

#[derive(serde::Deserialize)]
pub enum ClientMessage {

}

pub struct Client {
    socket: JsonClient<ClientMessage, RoomUpdate>,

    handle: RoomHandle,
    update_stream: broadcast::Receiver<RoomUpdate>,

    id: Uuid,
}

impl Client {
    pub async fn spawn_new(handle: RoomHandle, socket: WebSocket, id: Uuid) {
        let update_stream = handle.subscribe(id).await;
        let mut client = Self {
            socket: JsonClient::new(socket),
            handle,
            update_stream,
            id,
        };
        drop(tokio::spawn(async move { client.run().await }));
    }

    async fn run(&mut self) {
        let Err(err) = self.run_inner().await else { return };
        eprintln!("error produced in client: {err}");
    }

    async fn run_inner(&mut self) -> eyre::Result<()> {
        loop {
            todo!()
        }

        Ok(())
    }
}
