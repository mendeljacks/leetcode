use std::collections::HashSet;

pub struct Solution {}
enum Cell {
    SingleValue(char),
    MultiValue(HashSet<char>),
}
type Board = Vec<Vec<char>>;
type Progress = Vec<Vec<Cell>>;

fn board_to_progress(board: &mut Board) -> Progress {
    board
        .into_iter()
        .map(|el| {
            el.into_iter()
                .map(|chr| {
                    if *chr == '.' {
                        Cell::MultiValue(['1', '2', '3', '4', '5', '6', '7', '8', '9'].into())
                    } else {
                        Cell::SingleValue(*chr)
                    }
                })
                .collect()
        })
        .collect()
}

fn remove_from_row(progress: &mut Progress, row: usize, chr: &char) -> Progress {
    for (j, _) in (&progress[row]).into_iter().enumerate() {
        progress[row][j] = match progress[row][j] {
            Cell::SingleValue(chr) => Cell::SingleValue(chr),
            Cell::MultiValue(mut val) => {
                val.remove(&chr);
                Cell::MultiValue(val)
            }
        };
    }
    *progress
}
fn remove_from_col(progress: &mut Progress, col: i32, chr: char) {}
fn remove_from_sub(progress: &mut Progress, sub: i32, chr: char) {}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut progress: Vec<Vec<Cell>> = board_to_progress(board);

        for (i, row) in (&progress).iter().enumerate() {
            for (j, mut cell) in row.iter().enumerate() {
                progress = match cell {
                    Cell::SingleValue(chr) => remove_from_row(&mut progress, i, chr),
                    // Cell::MultiValue(val) => remove_from_row(&mut progress, i, &'a'.into()),
                };
            }
        }

        // for each element of the board if empty make all options
        // for each non empty element remove all combinations from others row, col, sub
        // look for items with one possibility and upgrade entries removing any additional invalid possibilities
        // look for items with cardinality matching number of boxes
        // look for items with only one option for a number in the row, col, sub
    }
}
