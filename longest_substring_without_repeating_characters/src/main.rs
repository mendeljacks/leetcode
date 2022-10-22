mod longest_substring_without_repeating_characters;
use longest_substring_without_repeating_characters::Solution;

fn main() {
    let tests = [
        // ("a", 1),
        // ("bbbbb", 1),
        // ("ab", 2),
        // ("abcabcbb", 3),
        // ("pwwkew", 3),
        // ("abc", 3),
        // ("abca", 3),
        // ("aabc", 3),
        // ("abcbc", 3),
        ("abcabcd", 4),
    ];

    for i in 0..tests.len() {
        let result = Solution::length_of_longest_substring(tests[i].0.into());
        println!("{}: expected {} got {}", tests[i].0, tests[i].1, result);
    }
}
