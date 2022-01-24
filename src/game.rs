use crate::{grid::Grid, tetromino::*, BLOCK_SIZE, math::Rotation};
use piston_window::*;
use rand::prelude::*;

struct FallingTetromino {
    tetromino: &'static Grid,
    col: isize,
    row: isize,
    rotation: Rotation,
}

impl FallingTetromino {
    fn get(&self, i: usize, j: usize) -> Option<&Block> {
        let [i, j] = self.rotation * [
            i as isize - self.col - self.tetromino.cols() as isize / 2,
            j as isize - self.row - self.tetromino.rows() as isize / 2,
        ];
        self.tetromino.get(
            i + self.tetromino.cols() as isize / 2,
            j + self.tetromino.rows() as isize / 2,
        )
    }

    fn try_move(&mut self, dx: isize, dy: isize, board: &Grid) -> bool {
        for (i, j, falling_block) in self.tetromino.cell_indices() {
            let [i, j] = self.rotation.inverse() * [
                i as isize - self.col - self.tetromino.cols() as isize / 2,
                j as isize - self.row - self.tetromino.rows() as isize / 2,
            ];
            let [i, j] = [
                i + self.col + self.tetromino.cols() as isize / 2,
                j + self.row + self.tetromino.rows() as isize / 2,
            ];

            if falling_block.is_some() {
                if j as isize + self.row + dy >= board.rows() as isize
                    || i as isize + self.col + dx < 0
                    || i as isize + self.col + dx >= board.cols() as isize
                    || board
                        .get(i as isize + self.col + dx, j as isize + self.row + dy)
                        .is_some()
                {
                    return false;
                }
            }
        }

        self.col += dx;
        self.row += dy;

        true
    }

    fn rotate(&mut self, board: &Grid) {
        // TODO
        self.rotation = self.rotation.rotate_90deg_right();
    }
}

#[derive(Clone, Copy)]
pub enum GameEvent {
    Tick,
    MoveLeft,
    MoveRight,
    Rotate,
}

pub struct Game {
    falling: Option<FallingTetromino>,
    board: Grid,
}

impl Game {
    pub fn new(board_size: [usize; 2]) -> Self {
        Self {
            falling: None,
            board: Grid::new(board_size),
        }
    }

    pub fn spawn_tetromino(&mut self) {
        let mut rng = rand::thread_rng();
        let tetromino = STANDARD_TETROMINOS.choose(&mut rng).unwrap();
        let col = self.board.cols() / 2 - tetromino.cols() / 2;

        self.falling = Some(FallingTetromino {
            tetromino,
            col: col as isize,
            row: -(tetromino.rows() as isize),
            rotation: Rotation::identity(),
        });
    }

    fn persist_tetromino(&mut self, tetromino: FallingTetromino) {
        for (i, j, block) in tetromino.tetromino.cell_indices() {
            if block.is_some() {
                debug_assert!(self
                    .board
                    .get(i as isize + tetromino.col, j as isize + tetromino.row)
                    .is_none());

                self.board.set(
                    i as isize + tetromino.col,
                    j as isize + tetromino.row,
                    *block,
                );
            }
        }
    }

    pub fn handle_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::Tick => {
                if let Some(mut tetromino) = self.falling.take() {
                    if tetromino.try_move(0, 1, &self.board) {
                        self.falling = Some(tetromino);
                    } else {
                        self.persist_tetromino(tetromino);
                    }
                }

                if self.falling.is_none() {
                    self.spawn_tetromino();
                }
            }
            GameEvent::MoveLeft => {
                if let Some(tetromino) = &mut self.falling {
                    tetromino.try_move(-1, 0, &self.board);
                }
            }
            GameEvent::MoveRight => {
                if let Some(tetromino) = &mut self.falling {
                    tetromino.try_move(1, 0, &self.board);
                }
            }
            GameEvent::Rotate => {
                if let Some(tetromino) = &mut self.falling {
                    tetromino.rotate(&self.board);
                }
            }
            _ => {}
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        for (i, j, cell) in self.board.cell_indices() {
            let rect = [
                i as f64 * BLOCK_SIZE,
                j as f64 * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
            ];

            if let Some(tetromino) = &self.falling {
                if tetromino.get(i, j).is_some() {
                    rectangle([1.0, 0.0, 0.0, 1.0], rect, c.transform, g);
                    continue;
                }
            }

            if cell.is_some() {
                rectangle([1.0; 4], rect, c.transform, g);
            }
        }
    }
}
