use crate::common::Solution;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::convert::TryInto;

const WINDOW: isize = 25;

fn solve_a(nums: &[isize]) -> isize {
    let mut set: BTreeSet<isize> = nums[0..WINDOW].iter().copied().collect();
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

pub fn solve(lines: &[String]) -> Solution {
    let nums: Vec<isize> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();
    (solve_a(&nums).to_string(), "".to_string())
}
