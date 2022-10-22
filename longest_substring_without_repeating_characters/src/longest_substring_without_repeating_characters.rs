use std::{cmp::max, collections::HashMap};

pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut best = 0;
        let mut lookup: HashMap<char, usize> = HashMap::new();
        let mut left_cursor = 0;

        for (index, chr) in s.chars().enumerate() {
            if lookup.contains_key(&chr) {
                if lookup.len() > s.len() - index {
                    return max(lookup.len() as i32, best);
                }

                let first_index = *lookup.get(&chr).unwrap();
                for i in left_cursor..first_index {
                    let remove_chr = &s.chars().nth(i).unwrap();
                    lookup.remove(remove_chr);
                }
                left_cursor = first_index;
            }
            lookup.insert(chr, index);

            if lookup.len() > best as usize {
                best = lookup.len() as i32
            }
        }

        best
    }
}
