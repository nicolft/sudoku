use crate::db::Db;
use crate::sudoku::Grid;

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
pub fn create(_db: &State<Mutex<Db>>) -> &'static str {
    let grid = Grid::new(9);
    let _game = Game { grid };

    "a"
}
