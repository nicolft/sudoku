use crate::game::GameCommand;

use std::collections::HashMap;

use tokio::sync::mpsc;

pub struct Db {
    pub games: HashMap<u64, mpsc::UnboundedSender<GameCommand>>,
}

impl Db {
    pub fn new() -> Self {
        let games = HashMap::new();
        Db { games }
    }
}
