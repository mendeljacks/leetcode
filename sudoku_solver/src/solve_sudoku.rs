use std::{
    cell,
    collections::{HashMap, HashSet},
};

pub struct Solution {}
type Option = HashSet<u32>;
type Options = Vec<Vec<Option>>;
type Board = Vec<Vec<char>>;

fn remove_from_row(grid: &mut Options, row: usize, num: u32) {
    for cell in grid[row].iter_mut() {
        cell.remove(&num);
    }
}

fn remove_from_col(grid: &mut Options, col: usize, num: u32) {
    for row in grid.iter_mut() {
        row[col].remove(&num);
    }
}

fn remove_from_sub(grid: &mut Options, row: usize, col: usize, num: u32) {
    for (i, r) in grid.iter_mut().enumerate() {
        for (j, cell) in (*r).iter_mut().enumerate() {
            if row / 3 == i / 3 && col / 3 == j / 3 {
                cell.remove(&num);
            }
        }
    }
}

fn walk_board(board: &mut Board, options: &mut Options) {
    for (i, row) in board.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if *cell != '.' {
                // for each non empty element remove all combinations from others row, col, sub
                options[i][j].drain();
                remove_from_row(options, i, (*cell).to_digit(10).unwrap());
                remove_from_col(options, j, (*cell).to_digit(10).unwrap());
                remove_from_sub(options, i, j, (*cell).to_digit(10).unwrap());
            }
        }
    }

    // look for items with one possibility and upgrade entries removing any additional invalid possibilities
    for (i, row) in board.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if options[i][j].len() == 1 {
                board[i][j] = std::char::from_u32(options[i][j].take(&1).unwrap()).unwrap();
                options[i][j].drain();
                remove_from_row(options, i, (*cell).to_digit(10).unwrap());
                remove_from_col(options, j, (*cell).to_digit(10).unwrap());
                remove_from_sub(options, i, j, (*cell).to_digit(10).unwrap());
            }
        }
    }

    // look for items with cardinality matching number of boxes
    // look for items with only one option for a number in the row, col, sub
}

impl Solution {
    pub fn solve_sudoku(board: &mut Board) {
        // for each element of the board if empty make all options
        let mut options: Options = vec![vec![HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]); 9]; 9];

        walk_board(&mut board, &mut options);

        for i in options {
            println!("{:?}", i);
            println!("");
        }
    }
}
