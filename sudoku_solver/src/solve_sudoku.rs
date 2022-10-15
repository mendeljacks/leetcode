use std::collections::{HashMap, HashSet};

pub struct Solution {}
type HSu32 = HashSet<u32>;
type Options = Vec<Vec<HSu32>>;
type Board = Vec<Vec<char>>;

fn solved(options: &Options) -> bool {
    for row in options.into_iter() {
        for cell in row.into_iter() {
            if cell.len() != 1 {
                return false;
            }
        }
    }
    return true;
}

fn hsu32_to_string(hs: &HashSet<u32>) -> String {
    hs.iter()
        .collect::<Vec<&u32>>()
        .iter()
        .map(|el| std::char::from_digit(**el, 10).unwrap())
        .collect::<Vec<char>>()
        .into_iter()
        .collect()
}

fn handle_preemptive_sets(options: &mut Options) {
    //  hashmap = { "235": [(i,j), (i2,j2)] };

    let mut lookup: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

    for (i, row) in options.into_iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            // number of cells with equal value match number of values

            let string2: String = hsu32_to_string(cell);

            lookup.entry(string2).or_insert(vec![]).push((i, j));
        }
    }

    for (key, vals) in &lookup {
        // add other keys in lookup object which have subset of chars
        let subset_keys: Vec<String> = lookup
            .keys()
            .filter(|el| {
                let el2: String = el.to_string();
                el2 != *key && el2.chars().all(|c| key.contains(c))
            })
            .map(|el| el.to_string())
            .collect();
        let mut subset_vals = subset_keys
            .iter()
            .map(|el| lookup.get(el).unwrap().to_vec())
            .collect::<Vec<Vec<(usize, usize)>>>();
        // let vals be unique of vals with appended subset_vals
        let vals = vals
            .append(&mut subset_vals)
            .collect::<Vec<(usize, usize)>>()
            .into_iter()
            .collect::<HashSet<(usize, usize)>>()
            .into_iter()
            .collect::<Vec<(usize, usize)>>();
        for i in 0..9 {
            let characters = key.chars();
            for chr in characters {
                let num = chr.to_digit(10).unwrap();

                let row = &mut options[i];
                for (j, cell) in row.iter_mut().enumerate() {
                    // If same row preemptive set
                    if key.len() == vals.iter().filter(|el| el.0 == i).count()
                        && !vals.contains(&(i, j))
                    {
                        cell.remove(&num);
                    }
                }
            }
        }

        // If same col preemptive set
        for j in 0..9 {
            if key.len() == vals.iter().filter(|el| el.1 == j).count() {
                let characters = key.chars();
                for chr in characters {
                    let u = chr.to_digit(10).unwrap();

                    for (i, row) in options.iter_mut().enumerate() {
                        let cell = &mut row[j];
                        if !vals.contains(&(i, j)) {
                            cell.remove(&u);
                        }
                    }
                }
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                // If same sub preemptive set
                if key.len()
                    == vals
                        .iter()
                        .filter(|val| val.0 / 3 == i && val.1 / 3 == j)
                        .count()
                {
                    let characters = key.chars();
                    for chr in characters {
                        let u = chr.to_digit(10).unwrap();

                        for (ii, row) in options.iter_mut().enumerate() {
                            for (jj, cell) in row.iter_mut().enumerate() {
                                if !vals.contains(&(ii, jj)) && ii / 3 == i && jj / 3 == j {
                                    cell.remove(&u);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // look for items with only one option for a number in the row, col, sub
    for i in 0..9 {
        for digit in 0..9 {
            let mut count = 0;
            let mut last = (0, 0);
            for (j, cell) in options[i].iter().enumerate() {
                if cell.contains(&(digit + 1)) {
                    count += 1;
                    last = (i, j);
                }
            }
            if count == 1 {
                options[last.0][last.1] = HSu32::from([digit + 1]);
            }
        }
    }

    for j in 0..9 {
        for digit in 0..9 {
            let mut count = 0;
            let mut last = (0, 0);
            for (i, row) in options.iter().enumerate() {
                let cell = &row[j];
                if cell.contains(&(digit + 1)) {
                    count += 1;
                    last = (i, j);
                }
            }
            if count == 1 {
                options[last.0][last.1] = HSu32::from([digit + 1]);
            }
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            for digit in 0..9 {
                let mut count = 0;
                let mut last = (0, 0);
                for (ii, row) in options.iter().enumerate() {
                    for (jj, cell) in row.iter().enumerate() {
                        if cell.contains(&(digit + 1)) && ii / 3 == i && jj / 3 == j {
                            count += 1;
                            last = (ii, jj);
                        }
                    }
                }
                if count == 1 {
                    options[last.0][last.1] = HSu32::from([digit + 1]);
                }
            }
        }
    }
}

fn initial_scan(board: &Board, options: &mut Options) {
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

    while !solved(options) {
        print!("\x1B[2J\x1B[1;1H");

        println!("=====================");
        for (i, row) in options.into_iter().enumerate() {
            if i % 3 == 0 {
                println!("---------------------------------------------------------------------------------------------------------------------");
            }
            println!(
                "{0: <10}  {1: <10}  {2: <10} | {3: <10}  {4: <10}  {5: <10} | {6: <10}  {7: <10}  {8: <10}",
                hsu32_to_string(&row[0]),
                hsu32_to_string(&row[1]),
                hsu32_to_string(&row[2]),
                hsu32_to_string(&row[3]),
                hsu32_to_string(&row[4]),
                hsu32_to_string(&row[5]),
                hsu32_to_string(&row[6]),
                hsu32_to_string(&row[7]),
                hsu32_to_string(&row[8])
            );
        }
        println!("=====================");

        handle_preemptive_sets(options);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    println!("Solved!");

    // loop through options and set board
    for (i, row) in options.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            board[i][j] = std::char::from_digit(*cell.iter().next().unwrap(), 10).unwrap();
        }
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Board) {
        // for each element of the board if empty make all options
        let mut options: Options = vec![vec![HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]); 9]; 9];

        walk_board(board, &mut options);
    }
}
