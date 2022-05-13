use crate::sudoku::Grid;
use rocket::get;

struct Game {
    grid: Grid,
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/create")]
pub fn create() -> &'static str {
    let grid = Grid::new(9);
    let _game = Game { grid };

    "a"
}
