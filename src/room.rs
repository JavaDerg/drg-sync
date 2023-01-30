use bitflags::bitflags;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

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

pub struct Room {
    recv: mpsc::Receiver<(Uuid, RoomMessage)>,

    update_sender: broadcast::Sender<RoomUpdate>,

    users: HashMap<Uuid, String>,
    perms: HashMap<Uuid, ClientPerms>,

    death_ticket: Option<Arc<()>>,
}

impl Room {
    pub fn new() -> RoomHandle {
        RoomHandle::new()
    }

    pub async fn run(&mut self) {}
}

pub enum RoomMessage {}
#[derive(Clone)]
pub enum RoomUpdate {}

#[derive(Clone)]
pub struct RoomHandle {
    sender: mpsc::Sender<(Uuid, RoomMessage)>,
}

impl RoomHandle {
    fn new() -> Self {
        let (sender, recv) = mpsc::channel(64);
        let (update_sender, _) = broadcast::channel(128);

        let mut room = Room {
            recv,
            update_sender,
            users: Default::default(),
            perms: Default::default(),
            death_ticket: None,
        };
        let _ = tokio::spawn(async move { room.run().await });

        Self { sender }
    }
}
