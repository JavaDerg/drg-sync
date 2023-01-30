use crate::room::RoomHandle;
use std::collections::{BTreeMap, HashMap};
use std::sync::Weak;
use tokio::sync::mpsc;
use tokio::time::Instant;

pub struct RoomManager {
    recv: mpsc::Receiver<ManagerMessage>,

    rooms: HashMap<String, RoomHandle>,
    timeouts: BTreeMap<Instant, RoomHandle>,
}

impl RoomManager {
    async fn run(&mut self) {}
}

struct TimeoutHandle {
    signal: Weak<()>,
    kill_sig: RoomHandle,
}

enum ManagerMessage {
}

#[derive(Clone)]
pub struct RoomManagerHandle {
    sender: mpsc::Sender<ManagerMessage>,
}

impl RoomManagerHandle {
    fn new() -> Self {
        let (sender, recv) = mpsc::channel(128);

        let mut manager = RoomManager {
            recv,
            rooms: Default::default(),
            timeouts: Default::default(),
        };
        tokio::spawn(async move { manager.run() });

        Self {
            sender,
        }
    }
}
