use crate::common::Solution;

fn solve_for(init: &[usize], target: usize) -> usize {
    let turn0 = init.len() - 1;
    let mut next: usize = *init.last().unwrap();
    let mut last_seen: Vec<usize> = vec![target * 2; target];
    for (i, n) in init.iter().enumerate() {
        last_seen[*n] = i;
    }

    for turn in turn0..target - 1 {
        let ls = &mut last_seen[next];
        next = if *ls < target { turn - *ls } else { 0 };
        *ls = turn;
    }
    next
}

pub fn solve(lines: &[String]) -> Solution {
    let init: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    (
        solve_for(&init, 2020).to_string(),
        solve_for(&init, 30000000).to_string(),
    )
}
