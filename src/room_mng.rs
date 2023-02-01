use crate::client::Client;
use crate::room::{Room, RoomHandle};
use axum::extract::ws::WebSocket;
use std::collections::{BTreeMap, HashMap};
use std::sync::Weak;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::Instant;
use uuid::Uuid;

pub struct RoomManager {
    recv: mpsc::Receiver<ManagerMessage>,

    rooms: HashMap<String, RoomHandle>,
    timeouts: BTreeMap<Instant, TimeoutHandle>,

    room_lookup: HashMap<Uuid, RoomHandle>,

    unreg: mpsc::Sender<(Uuid, Weak<()>)>,
    unreg_recv: mpsc::Receiver<(Uuid, Weak<()>)>,
}

impl RoomManager {
    pub fn new() -> RoomManagerHandle {
        RoomManagerHandle::new()
    }

    async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(msg) = self.recv.recv() => self.handle_msg(msg).await,
                Some(urg) = self.unreg_recv.recv() => self.handle_unreg(urg).await,
                else => break,
            }
            todo!("also handle timeouts")
        }
    }

    async fn handle_msg(&mut self, msg: ManagerMessage) {
        match msg {
            ManagerMessage::JoinOrMake { room, id, socket } => {
                self.join_or_make(room, id, socket).await
            }
        }
    }

    async fn handle_unreg(&mut self, (id, signal): (Uuid, Weak<()>)) {
        if signal.strong_count() == 0 {
            return;
        }

        let Some(room) = self.room_lookup.get(&id).cloned() else { return };

        let mut time = Instant::now() + Duration::from_secs(60 * 5);
        while self.timeouts.contains_key(&time) {
            time += Duration::from_nanos(1);
        }
        self.timeouts.insert(time, TimeoutHandle {
            signal,
            room,
        });
    }

    async fn join_or_make(&mut self, room: String, id: Uuid, socket: WebSocket) {
        let room = self.rooms.get(&room).cloned().unwrap_or_else(|| self.create_room());

        // we await instead of letting it run in another task
        // to avoid race conditions in the timeout system
        Client::spawn_new(room.clone(), socket, id).await;
    }

    fn create_room(&mut self) -> RoomHandle {
        let id = Uuid::new_v4();

        let unreg = self.unreg.clone();
        let room = Room::new(Box::new(move |signal| {
            let unreg = unreg.clone();

            drop(tokio::spawn(async move {
                let _ = unreg.send((id, signal)).await;
            }));
        }));

        room
    }
}

struct TimeoutHandle {
    signal: Weak<()>,
    room: RoomHandle,
}

enum ManagerMessage {
    JoinOrMake {
        room: String,
        id: Uuid,
        socket: WebSocket,
    },
}

#[derive(Clone)]
pub struct RoomManagerHandle {
    sender: mpsc::Sender<ManagerMessage>,
}

impl RoomManagerHandle {
    fn new() -> Self {
        let (sender, recv) = mpsc::channel(128);
        let (unreg, unreg_recv) = mpsc::channel(16);

        let mut manager = RoomManager {
            recv,
            rooms: Default::default(),
            timeouts: Default::default(),
            room_lookup: Default::default(),
            unreg,
            unreg_recv,
        };
        tokio::spawn(async move { manager.run().await });

        Self { sender }
    }

    pub async fn join_or_make(&self, room: String, id: Uuid, socket: WebSocket) {
        self.sender
            .send(ManagerMessage::JoinOrMake { room, id, socket })
            .await
            .map_err(|_| mpsc::error::SendError(()))
            .expect("the room manager should not shutdown before the webserver does");
    }

    pub async fn enqueue_death(&self, room: RoomHandle) {
        todo!()
    }
}
