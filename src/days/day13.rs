use crate::common::Solution;
use std::collections::HashSet;

fn solve_a(init: u128, buses: &HashSet<u128>) -> u128 {
    let busid = buses
        .into_iter()
        .min_by_key(|b| **b - (init % **b))
        .unwrap();
    let wait = busid - (init % busid);
    busid * wait
}

fn solve_b(mut requirements: Vec<(usize, u128)>) -> u128 {
    requirements.sort_by_key(|(_, busid)| u128::MAX - busid);

    dbg!(&requirements);

    let remainders: Vec<u128> = requirements
        .iter()
        .map(|(t_offset, busid)| (*busid - (*t_offset as u128 % *busid)) % *busid)
        .collect();

    dbg!(&remainders);

    let mut x = remainders[0];
    let mut stepsize = requirements[0].1 as usize;
    for (i, ai) in remainders.iter().enumerate().skip(1) {
        let ni = requirements[i].1;

        dbg!(i, x, ai, ni, stepsize);

        for x_next in (x..).step_by(stepsize) {
            if x_next % ni == *ai {
                stepsize *= ni as usize;
                x = x_next;
                break;
            }
        }
    }

    x
}

pub fn solve(lines: &[String]) -> Solution {
    let init: u128 = lines[0].parse().unwrap();
    let requirements: Vec<(usize, u128)> = lines[1]
        .split(",")
        .enumerate()
        .filter(|(_, b)| b != &"x")
        .map(|(i, b)| (i, b.parse().unwrap()))
        .collect();
    let buses: HashSet<u128> = requirements.iter().map(|(_, v)| v).copied().collect();
    (
        solve_a(init, &buses).to_string(),
        solve_b(requirements).to_string(),
    )
}
