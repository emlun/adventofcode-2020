use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_binary(result: usize, c: char, zero_char: char) -> usize {
    if c == zero_char {
        result * 2
    } else {
        (result * 2 + 1)
    }
}

fn solve_a(passes: &[String]) -> usize {
    passes
        .iter()
        .map(|s| {
            let row: usize = s[0..7]
                .chars()
                .fold(0, |result, c| parse_binary(result, c, 'F'));
            let col: usize = s[7..]
                .chars()
                .fold(0, |result, c| parse_binary(result, c, 'L'));
            row * 8 + col
        })
        .max()
        .unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), "".to_string())
}
