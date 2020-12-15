use crate::common::Solution;
use std::collections::HashMap;

fn solve_for(lines: &[String], target: usize) -> usize {
    let init: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    let mut turn = init.len() - 1;

    let mut next: usize = init[turn];
    let mut last_seen: HashMap<usize, usize> = init
        .into_iter()
        .take(turn)
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect();

    loop {
        if turn + 1 == target {
            return next;
        }

        if let Some(last_seen_at) = last_seen.get_mut(&next) {
            next = turn - *last_seen_at;
            *last_seen_at = turn;
        } else {
            last_seen.insert(next, turn);
            next = 0;
        }

        turn += 1;
    }
}

fn solve_a(lines: &[String]) -> usize {
    solve_for(lines, 2020)
}

fn solve_b(lines: &[String]) -> usize {
    solve_for(lines, 30000000)
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b(lines).to_string())
}
