use std::{
    cell,
    collections::{HashMap, HashSet},
};

pub struct Solution {}
type HSu32 = HashSet<u32>;
type Options = Vec<Vec<HSu32>>;
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

fn remove_singletons(board: &mut Board, options: &mut Options) {
    // ssingletons and upgrade entries removing any additional invalid possibilities
    for (i, row) in board.into_iter().enumerate() {
        for (j, mut cell) in row.iter_mut().enumerate() {
            if options[i][j].len() == 1 {
                let u = options[i][j].drain().next().unwrap();
                let mut chr = std::char::from_u32(u).unwrap();
                cell = &mut chr;
                remove_from_row(options, i, u);
                remove_from_col(options, j, u);
                remove_from_sub(options, i, j, u);
            }
        }
    }
}

fn all_equal<T: PartialEq>(iter: impl IntoIterator<Item = T>) -> bool {
    let mut iter = iter.into_iter();

    match iter.next() {
        None => true,
        Some(first) => iter.all(|x| x == first),
    }
}

fn remove_preemptive(options: &mut Options) {
    // look for items with cardinality matching number of boxes preemptive set

    // let st = {
    //     "235": [(i,j), (i2,j2)]
    // }

    let mut lookup: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

    for (i, row) in options.into_iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            // number of cells with equal value match number of values

            let string2: String = cell
                .iter()
                .collect::<Vec<&u32>>()
                .iter()
                .map(|el| std::char::from_u32(**el).unwrap())
                .collect::<Vec<char>>()
                .into_iter()
                .collect();

            lookup.entry(string2).or_insert(vec![]).push((i, j));
        }
    }

    for (k, v) in lookup {
        // If same row preemptive set
        if k.len() == v.len() && all_equal(v.iter().map(|el| el.0)) {
            let character = k.chars();
            for c in character {
                let u = c.to_digit(10).unwrap();

                let row = &mut options[v[0].0];
                for (j, cell) in row.iter_mut().enumerate() {
                    if cell.contains(&u) && !v.contains(&(v[0].0, j)) {
                        cell.remove(&u);
                    }
                }
            }
        }

        // If same col preemptive set
        if k.len() == v.len() && all_equal(v.iter().map(|el| el.1)) {
            let character = k.chars();
            for c in character {
                let u = c.to_digit(10).unwrap();

                for (i, row) in options.iter_mut().enumerate() {
                    let cell = &mut row[v[0].1];
                    if cell.contains(&u) && !v.contains(&(i, v[0].1)) {
                        cell.remove(&u);
                    }
                }
            }
        }

        // If same sub preemptive set
        if k.len() == v.len()
            && (all_equal(v.iter().map(|el| el.0 / 3)) || all_equal(v.iter().map(|el| el.1 / 3)))
        {
            let character = k.chars();
            for c in character {
                let u = c.to_digit(10).unwrap();

                for (i, row) in options.iter_mut().enumerate() {
                    for (j, cell) in row.iter_mut().enumerate() {
                        if cell.contains(&u)
                            && !v.contains(&(i, j))
                            && v[0].0 / 3 == i / 3
                            && v[0].1 / 3 == j / 3
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

fn walk_board(board: &mut Board, options: &mut Options) {
    // initial scan
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

    while !solved(board) {
        remove_singletons(board, options);
        remove_preemptive(options)
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
