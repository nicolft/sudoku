use crate::db::Db;
use crate::sudoku::Grid;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use rocket::{get, State};

pub struct Game {
    grid: Grid,
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/create")]
pub fn create(db: &State<Mutex<Db>>) -> String {
    let grid = Grid::new(9);
    let game = Game { grid };
    let id = next_game_id();

    db.lock().unwrap().games.insert(id, game);

    format!("{}", id)
}

fn next_game_id() -> u64 {
    // Create a game ID
    static ID: AtomicU64 = AtomicU64::new(0);

    // Get the next one
    ID.fetch_add(1, Ordering::Relaxed)
}
