use std::{
    cell,
    collections::{HashMap, HashSet},
};

pub struct Solution {}
type HSu32 = HashSet<u32>;
type Options = Vec<Vec<HSu32>>;
type Board = Vec<Vec<char>>;

fn solved(board: &Board) -> bool {
    for row in board.into_iter() {
        for cell in row.into_iter() {
            if *cell == '.' {
                return false;
            }
        }
    }
    return true;
}

fn all_equal<T: PartialEq>(iter: impl IntoIterator<Item = T>) -> bool {
    let mut iter = iter.into_iter();

    match iter.next() {
        None => true,
        Some(first) => iter.all(|x| x == first),
    }
}

fn handle_preemptive_sets(options: &mut Options) {
    //  hashmap = { "235": [(i,j), (i2,j2)] };

    let mut lookup: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

    for (i, row) in options.into_iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            // number of cells with equal value match number of values

            let string2: String = cell
                .iter()
                .collect::<Vec<&u32>>()
                .iter()
                .map(|el| std::char::from_digit(**el, 10).unwrap())
                .collect::<Vec<char>>()
                .into_iter()
                .collect();

            lookup.entry(string2).or_insert(vec![]).push((i, j));
        }
    }

    for (key, vals) in lookup {
        // If same row preemptive set
        if key.len() == vals.len() && all_equal(vals.iter().map(|el| el.0)) {
            let characters = key.chars();
            for chr in characters {
                let num = chr.to_digit(10).unwrap();

                let row = &mut options[vals[0].0];
                for (j, cell) in row.iter_mut().enumerate() {
                    if cell.contains(&num) && !vals.contains(&(vals[0].0, j)) {
                        cell.remove(&num);
                    }
                }
            }
        }

        // If same col preemptive set
        if key.len() == vals.len() && all_equal(vals.iter().map(|el| el.1)) {
            let characters = key.chars();
            for chr in characters {
                let u = chr.to_digit(10).unwrap();

                for (i, row) in options.iter_mut().enumerate() {
                    let cell = &mut row[vals[0].1];
                    if cell.contains(&u) && !vals.contains(&(i, vals[0].1)) {
                        cell.remove(&u);
                    }
                }
            }
        }

        // If same sub preemptive set
        if key.len() == vals.len()
            && (all_equal(vals.iter().map(|el| el.0 / 3))
                || all_equal(vals.iter().map(|el| el.1 / 3)))
        {
            let characters = key.chars();
            for chr in characters {
                let u = chr.to_digit(10).unwrap();

                for (i, row) in options.iter_mut().enumerate() {
                    for (j, cell) in row.iter_mut().enumerate() {
                        if cell.contains(&u)
                            && !vals.contains(&(i, j))
                            && vals[0].0 / 3 == i / 3
                            && vals[0].1 / 3 == j / 3
                        {
                            cell.remove(&u);
                        }
                    }
                }
            }
        }
    }

    // look for items with only one option for a number in the row, col, sub
}

fn initial_scan(board: &mut Board, options: &mut Options) {
    // initial scan
    for (i, row) in board.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if *cell != '.' {
                options[i][j].drain();
                options[i][j].insert(cell.to_digit(10).unwrap());
            }
        }
    }
}

fn walk_board(board: &mut Board, options: &mut Options) {
    initial_scan(board, options);

    while !solved(board) {
        println!("=====================");
        for row in options.into_iter() {
            for cell in row.into_iter() {
                print!("{0: <10} ", cell);
            }
            println!();
        }
        handle_preemptive_sets(options);
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Board) {
        // for each element of the board if empty make all options
        let mut options: Options = vec![vec![HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]); 9]; 9];

        walk_board(board, &mut options);

        for i in options {
            println!("{:?}", i);
            println!("");
        }
    }
}
