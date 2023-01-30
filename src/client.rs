use crate::room::{RoomHandle, RoomUpdate};
use axum::extract::ws::WebSocket;
use tokio::sync::broadcast;
use uuid::Uuid;

mod json;

pub struct Client {
    socket: WebSocket,

    handle: RoomHandle,
    updates: Option<broadcast::Receiver<RoomUpdate>>,

    id: Uuid,
}

impl Client {
    pub async fn new(handle: RoomHandle, socket: WebSocket, id: Uuid) {
        let mut client = Self {
            socket,
            handle,
            updates: None,
            id,
        };
        let _ = tokio::spawn(async move { client.run().await });
    }

    async fn run(&mut self) {
    }
}
