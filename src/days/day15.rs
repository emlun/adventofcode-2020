use crate::common::Solution;
use std::collections::BTreeSet;
use std::collections::HashMap;

fn solve_a(lines: &[String]) -> usize {
    let mut history: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();

    loop {
        let last = history.last().unwrap();

        if history.len() == 2020 {
            return *last;
        }

        let mut found = false;
        for i in (0..history.len() - 1).rev() {
            if history[i] == *last {
                history.push(history.len() - i - 1);
                found = true;
                break;
            }
        }
        if !found {
            history.push(0);
        }
    }
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), "".to_string())
}
