use crate::common::Solution;
use std::collections::HashSet;

pub fn solve(lines: &[String]) -> Solution {
    let init: usize = lines[0].parse().unwrap();
    let mut buses: HashSet<usize> = lines[1]
        .split(",")
        .filter(|b| b != &"x")
        .map(|b| b.parse().unwrap())
        .collect();
    dbg!(&buses);
    let busid = buses.into_iter().min_by_key(|b| b - (init % b)).unwrap();
    dbg!(busid);
    let wait = busid - (init % busid);
    dbg!(wait);
    let a_solution = busid * wait;

    (a_solution.to_string(), "".to_string())
}
