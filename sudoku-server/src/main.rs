#[allow(dead_code)]
mod sudoku;

use sudoku::{Grid, CellValue};

fn main() {
    let mut grid = Grid::new(3);
    println!("{}", grid);

    grid.put(0,1,CellValue::One);
    println!("{}", grid);

    grid.put(0,1,CellValue::Empty);
    println!("{}", grid);
}
