use crate::common::Solution;
use std::collections::HashSet;

fn solve_a(lines: &[String]) -> usize {
    let mut tiles: HashSet<(i32, i32)> = HashSet::new();

    for mut line in lines.iter().map(|s| s.as_str()) {
        let (mut hexx, mut hexy) = (0, 0);
        while !line.is_empty() {
            if let Some(remaining) = line.strip_prefix("nw") {
                hexx -= 1;
                hexy -= 1;
                line = remaining;
            } else if let Some(remaining) = line.strip_prefix("ne") {
                hexy -= 1;
                line = remaining;
            } else if let Some(remaining) = line.strip_prefix("sw") {
                hexy += 1;
                line = remaining;
            } else if let Some(remaining) = line.strip_prefix("se") {
                hexx += 1;
                hexy += 1;
                line = remaining;
            } else if let Some(remaining) = line.strip_prefix("w") {
                hexx -= 1;
                line = remaining;
            } else if let Some(remaining) = line.strip_prefix("e") {
                hexx += 1;
                line = remaining;
            } else {
                unreachable!();
            }
        }
        if tiles.contains(&(hexx, hexy)) {
            tiles.remove(&(hexx, hexy));
        } else {
            tiles.insert((hexx, hexy));
        }
    }
    tiles.len()
}

fn solve_b(lines: &[String]) -> usize {
    0
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b(lines).to_string())
}
