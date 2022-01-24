use crate::{grid::Grid, tetromino::*, BLOCK_SIZE, math::Rotation};
use piston_window::*;
use rand::prelude::*;

#[derive(Clone)]
struct FallingTetromino {
    tetromino: &'static Grid,
    col: isize,
    row: isize,
    rotation: Rotation,
}

impl FallingTetromino {
    fn local_to_global(&self, [i, j]: [isize; 2]) -> [isize; 2] {
        // Transpose to middle & invert rotation
        let [i, j] = self.rotation.inverse() * [
            i as isize - self.tetromino.cols() as isize / 2,
            j as isize - self.tetromino.rows() as isize / 2,
        ];
        // Transpose back and transpose by tetromino position
        [
            i + self.tetromino.cols() as isize / 2 + self.col,
            j + self.tetromino.rows() as isize / 2 + self.row,
        ]
    }

    fn global_to_local(&self, [i, j]: [isize; 2]) -> [isize; 2] {
        // Convert to local coordinates, transpose to middle & rotate
        let [i, j] = self.rotation * [
            i as isize - self.col - self.tetromino.cols() as isize / 2,
            j as isize - self.row - self.tetromino.rows() as isize / 2,
        ];
        // Transpose back
        [
            i + self.tetromino.cols() as isize / 2,
            j + self.tetromino.rows() as isize / 2,
        ]
    }

    fn get(&self, i: usize, j: usize) -> Option<&Block> {
        let [i, j] = self.global_to_local([i as isize, j as isize]);
        self.tetromino.get(i, j)
    }

    fn try_move(&mut self, dx: isize, dy: isize, board: &Grid) -> bool {
        for (i, j, falling_block) in self.tetromino.cell_indices() {
            let [i, j] = self.local_to_global([i as isize, j as isize]);

            if falling_block.is_some() {
                if j + dy >= board.rows() as isize
                    || i + dx < 0
                    || i + dx >= board.cols() as isize
                    || board.get(i + dx, j + dy).is_some()
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
        let mut new = self.clone();
        new.rotation = new.rotation.rotate_90deg_right();

        // minimum column offset required for the tetromino not to collide with walls
        let mut dx = 0;

        for (i, j, falling_block) in new.tetromino.cell_indices() {
            let [i, _] = new.local_to_global([i as isize, j as isize]);

            if falling_block.is_some() {
                if i < 0 {
                    dx = dx.max(-i);
                }

                if i >= board.cols() as isize {
                    dx = dx.min(board.cols() as isize - 1 - i);
                }
            }
        }

        new.col += dx;

        // test kicks
        for (kx, ky) in [(0, 0), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)] {
            if new.try_move(kx, ky, board) {
                *self = new;
                return;
            }
        }

        // if none of the kicks worked, *self is not assigned to and the rotation
        // is not performed
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
            let [i, j] = tetromino.local_to_global([i as isize, j as isize]);

            if block.is_some() {
                debug_assert!(self.board.get(i, j).is_none());
                self.board.set(i, j, *block);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tetromino_local_global() {
        let tetromino = FallingTetromino {
            col: 123,
            row: 345,
            rotation: Rotation::identity().rotate_90deg_right(),
            tetromino: &STANDARD_TETROMINOS[0],
        };

        assert_eq!(tetromino.global_to_local(tetromino.local_to_global([0, 0])), [0, 0]);
        assert_eq!(tetromino.local_to_global(tetromino.global_to_local([0, 0])), [0, 0]);
    }
}
