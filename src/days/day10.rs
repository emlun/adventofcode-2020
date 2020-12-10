use crate::common::Solution;
use std::collections::BTreeSet;

const WINDOW: usize = 25;

fn solve_a(nums: &[usize]) -> usize {
    let mut diff1 = 0;
    let mut diff3 = 0;
    for i in 1..nums.len() {
        match nums[i] - nums[i - 1] {
            1 => diff1 += 1,
            3 => diff3 += 1,
            _ => {}
        }
    }
    diff1 * diff3
}

pub fn solve(lines: &[String]) -> Solution {
    let mut nums: Vec<usize> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();
    nums.push(0);
    nums.sort_unstable();
    nums.push(nums.last().unwrap() + 3);
    let a_solution = solve_a(&nums);
    (a_solution.to_string(), "".to_string())
}
