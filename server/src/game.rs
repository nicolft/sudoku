use crate::sudoku::{Grid, Size, Value};

use tokio::sync::mpsc;

pub enum GameCommand {
    Place(Size, Size, Value),
    Shutdown,
}

pub struct GameCtl {
    pub game_tx: mpsc::UnboundedSender<GameCommand>,
    pub size: Size,
}

pub struct Game {
    grid: Grid,
    game_rx: mpsc::UnboundedReceiver<GameCommand>,
}

impl Game {
    pub fn new(game_rx: mpsc::UnboundedReceiver<GameCommand>, size: Size) -> Self {
        Game {
            grid: Grid::new(size),
            game_rx,
        }
    }

    pub async fn run(&mut self) {
        use GameCommand::*;

        while let Some(cmd) = self.game_rx.recv().await {
            match cmd {
                Place(row, col, value) => {
                    self.grid.place(row, col, value);
                }
                Shutdown => break,
            }
        }

        // TODO Make sure task won't run forever if a user dc's
    }
}
