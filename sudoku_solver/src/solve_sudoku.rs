use std::{cell, collections::HashSet};

pub struct Solution {}
type Option = HashSet<i32>;
type Options = Vec<Vec<Option>>;
type Board = Vec<Vec<char>>;

fn remove_from_row(grid: &mut Options, row: usize, num: i32) {
    for cell in grid[row].iter_mut() {
        cell.remove(&num);
    }
}

fn remove_from_col(grid: &mut Options, col: usize, num: i32) {
    for row in grid.iter_mut() {
        row[col].remove(&num);
    }
}

fn walk_board(board: &Board, options: &mut Options) {
    for (i, row) in board.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if *cell != '.' {
                options[i][j].drain();
                remove_from_row(options, i, *cell as i32);
                remove_from_col(options, j, *cell as i32);
                remove_from_sub(options, j, *cell as i32);
            }
        }
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Board) {
        let mut options: Options = vec![vec![HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]); 9]; 9];

        walk_board(&board, &mut options);

        println!("{:?}", options);
        // for each element of the board if empty make all options
        // for each non empty element remove all combinations from others row, col, sub
        // look for items with one possibility and upgrade entries removing any additional invalid possibilities
        // look for items with cardinality matching number of boxes
        // look for items with only one option for a number in the row, col, sub
    }
}
