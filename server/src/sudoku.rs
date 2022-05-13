use std::collections::HashMap;

// type Username = String;

#[derive(Debug)]
pub enum CellValue {
    Empty,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl std::fmt::Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>5}", format!("{:?}", self))
    }
}

// enum CellKind {
//     Standard(CellValue),
//     Multi(HashMap<Username, CellValue>),
// }

#[derive(Debug)]
struct Cell {
    // kind: CellKind
    value: CellValue,
}

impl Cell {
    fn new() -> Self {
        Self {
            value: CellValue::Empty,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct Grid {
    dimension: usize,
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(
        dimension: usize,
    ) -> Self {
        // Create grid with Empty cells.
        let mut grid = Vec::new();

        for _ in 0..dimension {
            let mut row = Vec::new();

            for _ in 0..dimension {
                row.push(Cell::new());
            }

            grid.push(row);
        }

        Self {
            dimension,
            grid,
        }
    }

    pub fn put(
        &mut self,
        row: usize,
        col: usize,
        value: CellValue,
    ) {
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
                s.push_str(&format!("{} ", self.grid[row][col]))
            }

            s.push_str("\n");
        }

        write!(f, "{}", s)
    }
}