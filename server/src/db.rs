use crate::game::GameCommand;

use dashmap::DashMap;
use tokio::sync::mpsc;

pub struct Db {
    pub games: DashMap<u64, mpsc::UnboundedSender<GameCommand>>,
}

impl Db {
    pub fn new() -> Self {
        let games = DashMap::new();
        Db { games }
    }
}
