use lazy_static::lazy_static;
use crate::grid::Grid;

#[derive(Clone, Copy)]
pub struct Block; // placeholder for future per-block data

fn parse_tetromino(s: &[&str]) -> Grid {
    let rows = s.len();
    debug_assert!(rows > 0);
    let cols = s[0].len();
    debug_assert!(cols > 0);

    let mut cells = vec![];
    for row in s {
        debug_assert!(row.len() == cols);
        cells.extend(row.chars().map(|c| match c {
            '#' => Some(Block),
            _ => None,
        }));
    }

    Grid::from_vec(cols, rows, cells)
}

lazy_static! {
    pub static ref STANDARD_TETROMINOS: Vec<Grid> = vec![
        parse_tetromino(&[
            " #  ",
            " #  ",
            " #  ",
            " #  ",
        ]),
        parse_tetromino(&[
            "##",
            "##",
        ]),
        parse_tetromino(&[
            " # ",
            " # ",
            "## ",
        ]),
        parse_tetromino(&[
            " # ",
            " # ",
            " ##",
        ]),
        parse_tetromino(&[
            "   ",
            "###",
            " # ",
        ]),
        parse_tetromino(&[
            " # ",
            " ##",
            "  #",
        ]),
        parse_tetromino(&[
            " # ",
            "## ",
            "#  ",
        ]),
    ];
}
