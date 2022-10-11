use std::collections::HashSet;

pub struct Solution {}

enum Cell {
    Char,
    Set,
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let progress: &mut Vec<Vec<Cell>> = board;
        for (i, row) in board.iter().enumerate() {
            for (j, mut cell) in row.iter().enumerate() {
                if *cell == '.' {
                    cell = [1, 2, 3, 4, 5, 6, 7, 8, 9];
                }
            }
        }

        // for each element of the board if empty make all options
        // for each non empty element remove all combinations from others row, col, sub
        // look for items with one possibility and upgrade entries removing any additional invalid possibilities
        // look for items with cardinality matching number of boxes
        // look for items with only one option for a number in the row, col, sub
    }
}
