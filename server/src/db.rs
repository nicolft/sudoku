use crate::webserver::Game;
use std::collections::HashMap;

pub struct Db {
    pub games: HashMap<u64, Game>,
}

impl Db {
    pub fn new() -> Self {
        let games = HashMap::new();
        Db { games }
    }
}
