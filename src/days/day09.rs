use crate::common::Solution;
use std::collections::BTreeSet;

const WINDOW: usize = 25;

fn solve_a(nums: &[usize]) -> usize {
    let mut set: BTreeSet<usize> = nums[0..WINDOW as usize].iter().copied().collect();
    for i in 0.. {
        let imax = (i + WINDOW) as usize;
        let j = nums[imax];
        if set
            .iter()
            .any(|n| j.checked_sub(*n).map(|d| set.contains(&d)).unwrap_or(false))
        {
            set.remove(&nums[i as usize]);
            set.insert(nums[imax]);
        } else {
            return j;
        }
    }
    unreachable!()
}

fn solve_b(nums: &[usize], a_solution: usize) -> usize {
    for seqlen in 2.. {
        let mut sum: usize = nums[0..seqlen].iter().sum();
        for i in 0..(nums.len() - seqlen) {
            let imax = i + seqlen;
            if sum == a_solution {
                return nums[i..imax].iter().min().unwrap() + nums[i..imax].iter().max().unwrap();
            } else {
                sum = sum - nums[i] + nums[imax];
            }
        }
    }
    unreachable!()
}

pub fn solve(lines: &[String]) -> Solution {
    let nums: Vec<usize> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();
    let a_solution = solve_a(&nums);
    (
        a_solution.to_string(),
        solve_b(&nums, a_solution).to_string(),
    )
}
