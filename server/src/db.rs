use crate::game::GameCtl;

use dashmap::DashMap;

pub struct Db {
    pub games: DashMap<u64, GameCtl>,
}

impl Db {
    pub fn new() -> Self {
        let games = DashMap::new();
        Db { games }
    }
}
