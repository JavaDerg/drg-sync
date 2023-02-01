mod timer;

use crate::room::timer::{Controller, PlayerEvent};
use bitflags::bitflags;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::sync::broadcast::Receiver;
use tokio::time::Interval;
use uuid::Uuid;
use crate::room_mng::RoomManagerHandle;

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct ClientPerms: u8 {
        const NONE        = 0b00000000;

        const REQ_ENQUEUE = 0b00000001;
        const MOD_QUEUE   = 0b00000010;

        const PAUSE_VIDEO = 0b00000100;
        const JMP_VIDEO   = 0b00001000;
        const MOD_VIDEO   = Self::PAUSE_VIDEO.bits()
                            | Self::JMP_VIDEO.bits();

        const ADMIN       = 0b11111111;
    }
}

pub type UnregisterCallback = Box<dyn Fn(Weak<()>) + Send + Sync + 'static>;

pub struct Room {
    recv: mpsc::Receiver<(Uuid, RoomMessage)>,

    update_sender: broadcast::Sender<RoomUpdate>,

    controller: Controller,

    users: HashMap<Uuid, String>,
    perms: HashMap<Uuid, ClientPerms>,

    death_ticket: Option<Arc<()>>,
    enqueue_death: UnregisterCallback,
}

impl Room {
    pub fn new(enqueue_death: UnregisterCallback) -> RoomHandle {
        RoomHandle::new(enqueue_death)
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                biased;

                event = self.controller.next_update() => self.broadcast_update(event).await,

                Some((id, msg)) = self.recv.recv() => self.handle_msg(id, msg).await,
                else => break,
            }
        }
    }

    async fn broadcast_update(&mut self, update: PlayerEvent) {
        if self.death_ticket.is_some() {
            return;
        }

        let _ = self.update_sender.send(RoomUpdate::Player(update));
    }

    async fn handle_msg(&mut self, id: Uuid, msg: RoomMessage) {
        match msg {
            RoomMessage::Subscribe { ret } => {
                let _ = ret.send(self.join_user(id));
            }
            RoomMessage::Unsubscribe(id) => {
                let _ = self.users.remove(&id);
                let _ = self.perms.remove(&id);

                if self.users.is_empty() {
                    let ticket = Arc::new(());
                    let ticket_ref = Arc::downgrade(&ticket);

                    self.death_ticket = Some(ticket);
                    (self.enqueue_death)(ticket_ref);
                }
            }
        }
    }

    fn join_user(&mut self, id: Uuid) -> Receiver<RoomUpdate> {
        // TODO: usernames
        if self.users.insert(id, "User".to_string()).is_some() {
            unreachable!("duplicate UUIDs should NEVER happen");
        }

        let _ = self.perms.insert(id, ClientPerms::NONE);

        // we don't want to die anymore in case we wanted to
        self.death_ticket = None;

        self.update_sender.subscribe()
    }
}

pub enum RoomMessage {
    Subscribe {
        ret: oneshot::Sender<Receiver<RoomUpdate>>,
    },
    Unsubscribe(Uuid),
}
#[derive(Clone, serde::Serialize)]
pub enum RoomUpdate {
    Player(PlayerEvent)
}

#[derive(Clone)]
pub struct RoomHandle {
    id: Uuid,
    sender: mpsc::Sender<(Uuid, RoomMessage)>,
}

impl RoomHandle {
    fn new(enqueue_death: UnregisterCallback) -> Self {
        let id = Uuid::new_v4();

        let (sender, recv) = mpsc::channel(64);
        let (update_sender, _) = broadcast::channel(128);

        let mut room = Room {
            recv,
            update_sender,
            controller: Controller::new(),
            users: Default::default(),
            perms: Default::default(),
            death_ticket: None,
            enqueue_death,
        };
        drop(tokio::spawn(async move { room.run().await }));

        Self { id, sender }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub async fn subscribe(&self, id: Uuid) -> broadcast::Receiver<RoomUpdate> {
        let (ret, rx) = oneshot::channel();
        self.sender
            .send((id, RoomMessage::Subscribe { ret }))
            .await
            .map_err(|_| mpsc::error::SendError(id))
            .expect("Channel should not be closed when we still hold a handle");
        rx.await
            .expect("Channel should return subscription while we still hold a handle")
    }
}
