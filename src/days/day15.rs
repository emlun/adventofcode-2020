use crate::common::Solution;

fn solve_for(lines: &[String], target: usize) -> usize {
    let init: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();

    let turn0 = init.len() - 1;
    let mut next: usize = *init.last().unwrap();
    let mut last_seen: Vec<Option<usize>> = vec![None; target];
    for (i, n) in init.into_iter().enumerate() {
        last_seen[n] = Some(i);
    }

    for turn in turn0..target - 1 {
        if let Some(last_seen_at) = &mut last_seen[next] {
            next = turn - *last_seen_at;
            *last_seen_at = turn;
        } else {
            last_seen[next] = Some(turn);
            next = 0;
        }
    }
    next
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
