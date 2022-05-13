use std::fmt::Write;

// type Username = String;

pub type Size = usize;
pub type Value = usize;

#[derive(Debug)]
struct Cell {
    // kind: CellKind
    value: Value,
}

impl Cell {
    fn new() -> Self {
        Self { value: 0 }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct Grid {
    dimension: Size,
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(dimension: Size) -> Self {
        // Create grid with Empty cells.
        let mut grid = Vec::new();

        for _ in 0..dimension {
            let mut row = Vec::new();

            for _ in 0..dimension {
                row.push(Cell::new());
            }

            grid.push(row);
        }

        Self { dimension, grid }
    }

    pub fn place(&mut self, row: Size, col: Size, value: Value) {
        if self.dimension < row || self.dimension < col {
            panic!("Row or column should be pre-validated.");
        }

        self.grid[row][col].value = value;
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("Grid: {dim} x {dim}\n", dim = self.dimension);

        for row in 0..self.dimension {
            s.push_str("| ");

            for col in 0..self.dimension {
                let _ = write!(s, "{} ", self.grid[row][col]);
            }

            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

fn _test_grid() {
    let mut grid = Grid::new(3);
    println!("{}", grid);

    grid.place(0, 1, 1);
    println!("{}", grid);

    grid.place(0, 1, 0);
    println!("{}", grid);
}
