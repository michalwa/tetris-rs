use crate::tetromino::Block;

pub struct Grid {
    cols: usize,
    rows: usize,
    cells: Box<[Option<Block>]>,
}

impl Grid {
    pub fn new([cols, rows]: [usize; 2]) -> Self {
        Self {
            cols,
            rows,
            cells: vec![None; cols * rows].into(),
        }
    }

    pub fn from_vec(cols: usize, rows: usize, cells: Vec<Option<Block>>) -> Self {
        debug_assert_eq!(cells.len(), cols * rows);

        Self {
            cols,
            rows,
            cells: cells.into(),
        }
    }

    pub const fn cols(&self) -> usize { self.cols }
    pub const fn rows(&self) -> usize { self.rows }

    pub fn get(&self, i: isize, j: isize) -> Option<&Block> {
        if i >= 0 && i < self.cols as isize && j >= 0 && j < self.rows as isize {
            self.cells[i as usize + j as usize * self.cols].as_ref()
        } else {
            None
        }
    }

    pub fn set(&mut self, i: isize, j: isize, block: Option<Block>) {
        if i >= 0 && i < self.cols as isize && j >= 0 && j < self.rows as isize {
            self.cells[i as usize + j as usize * self.cols] = block;
        }
    }

    pub fn cell_indices(&self) -> CellIndices {
        CellIndices {
            i: 0,
            grid: self,
        }
    }
}

pub struct CellIndices<'a> {
    i: usize,
    grid: &'a Grid,
}

impl<'a> Iterator for CellIndices<'a> {
    type Item = (usize, usize, &'a Option<Block>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.grid.cols * self.grid.rows { return None }

        let col = self.i % self.grid.cols;
        let row = self.i / self.grid.cols;
        let item = (col, row, &self.grid.cells[self.i]);

        self.i += 1;
        Some(item)
    }
}
