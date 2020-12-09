use crate::common::Solution;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::convert::TryInto;

const WINDOW: isize = 25;

fn solve_a(nums: &[isize]) -> isize {
    let mut set: BTreeSet<isize> = nums[0..WINDOW as usize].iter().copied().collect();
    let mut i: isize = 0;
    loop {
        let j = nums[(i + WINDOW) as usize];
        if set.iter().any(|n| set.contains(&(j - *n))) {
            set.remove(&nums[i as usize]);
            set.insert(nums[(i + WINDOW) as usize]);
            i += 1;
        } else {
            return j;
        }
    }
}

fn solve_b(nums: &[isize], a_solution: isize) -> isize {
    let mut set: BTreeSet<isize> = nums[0..WINDOW as usize].iter().copied().collect();
    let mut i: isize = 0;
    for seqlen in 2.. {
        for i in 0..(nums.len() - seqlen) {
            let imax = i + seqlen;
            if nums[i..imax].iter().sum::<isize>() == a_solution {
                return nums[i..imax].iter().min().unwrap() + nums[i..imax].iter().max().unwrap();
            }
        }
    }
    unreachable!()
}

pub fn solve(lines: &[String]) -> Solution {
    let nums: Vec<isize> = lines
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
