use crate::db::Db;
use crate::game::Game;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use rocket::{get, State};
use tokio::sync::mpsc;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/create")]
pub fn create(db: &State<Mutex<Db>>) -> String {
    let (game_tx, game_rx) = mpsc::unbounded_channel();
    let mut game = Game::new(game_rx);

    let id = next_game_id();
    db.lock().unwrap().games.insert(id, game_tx);

    tokio::spawn(async move {
        game.run().await;
    });

    format!("{}", id)
}

fn next_game_id() -> u64 {
    // Create a game ID
    static ID: AtomicU64 = AtomicU64::new(0);

    // Get the next one
    ID.fetch_add(1, Ordering::Relaxed)
}
