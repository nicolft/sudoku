use crate::webserver::Game;
use std::collections::HashMap;

pub struct Db {
    games: HashMap<u32, Game>,
}

impl Db {
    pub fn new() -> Self {
        let games = HashMap::new();
        Db { games }
    }
}
