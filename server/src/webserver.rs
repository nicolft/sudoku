use crate::db::Db;
use crate::game::{Game, GameCommand};
use crate::sudoku::CellValue;

use std::sync::atomic::{AtomicU64, Ordering};

use rocket::{get, post, State};
use tokio::sync::mpsc;

#[get("/")]
pub fn index() -> &'static str {
    "Sudoku! Wow!"
}

#[get("/create")]
pub fn create(db: &State<Db>) -> String {
    let (game_tx, game_rx) = mpsc::unbounded_channel();
    let mut game = Game::new(game_rx);

    let id = next_game_id();
    db.games.insert(id, game_tx);

    tokio::spawn(async move {
        game.run().await;
    });

    format!("{}", id)
}

#[get("/play")]
pub fn play() -> String {
    String::from("We are playing sudoku.")
}

#[post("/game/<id>/place")]
pub fn place(id: u64, db: &State<Db>) -> &'static str {
    match db.games.get(&id) {
        Some(game_tx) => {
            let cmd = GameCommand::Place(0, 0, CellValue::One);
            match game_tx.send(cmd) {
                Ok(_) => "Placed",
                Err(_) => "Game has already ended.", // TODO handle this
            }
        }
        None => "Invalid game id.", // TODO error handling
    }
}

fn next_game_id() -> u64 {
    // Create a game ID
    static ID: AtomicU64 = AtomicU64::new(0);

    // Get the next one
    ID.fetch_add(1, Ordering::Relaxed)
}
