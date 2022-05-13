use crate::db::Db;
use crate::game::{Game, GameCommand, GameCtl};
use crate::sudoku::{Size, Value};

use std::sync::atomic::{AtomicU64, Ordering};

use rocket::{get, post, State};
use tokio::sync::mpsc;

#[derive(Debug)]
enum WebServerError {
    Validation,
}

#[get("/")]
pub fn index() -> &'static str {
    "Sudoku! Wow!"
}

#[get("/create")]
pub fn create(db: &State<Db>) -> String {
    let size = 9;

    let (game_tx, game_rx) = mpsc::unbounded_channel();
    let mut game = Game::new(game_rx, size);

    let id = next_game_id();
    let ctl = GameCtl { game_tx, size };
    db.games.insert(id, ctl);

    tokio::spawn(async move {
        game.run().await;
    });

    format!("{}", id)
}

#[get("/play")]
pub fn play() -> String {
    String::from("We are playing sudoku.")
}

#[post("/game/<id>/place?<row>&<col>&<value>")]
pub fn place(db: &State<Db>, id: u64, row: Size, col: Size, value: Value) -> &'static str {
    match db.games.get(&id) {
        Some(ctl) => {
            if row > ctl.size || col > ctl.size || value > ctl.size {
                return "Invalid row, column, or value."; // TODO error handling
            }
            let cmd = GameCommand::Place(row, col, value);

            match ctl.game_tx.send(cmd) {
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
