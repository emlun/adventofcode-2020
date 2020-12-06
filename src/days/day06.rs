use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve_a(groups: &[HashSet<char>]) -> usize {
    groups.iter().map(|g| g.len()).sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let groups: Vec<HashSet<char>> = lines.iter().fold(vec![HashSet::new()], |mut result, line| {
        if line == "" {
            result.push(HashSet::new());
        } else {
            result.last_mut().unwrap().extend(line.chars());
        }
        result
    });

    (solve_a(&groups).to_string(), "".to_string())
}
