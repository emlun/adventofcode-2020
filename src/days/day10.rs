use crate::common::Solution;
use std::convert::TryInto;

fn solve_a(nums: &[u128]) -> u128 {
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

fn solve_b(nums: &[u128]) -> u128 {
    if nums.len() < 2 {
        nums.len().try_into().unwrap()
    } else {
        let midi = nums.len() / 2;
        let front = &nums[0..midi];
        let back = &nums[midi..];

        let mut arrangements = 0;

        for dfronti in 0..std::cmp::min(3, front.len()) {
            let fronti = front.len() - dfronti - 1;
            for backi in 0..std::cmp::min(3, back.len()) {
                if back[backi] - front[fronti] <= 3 {
                    arrangements += solve_b(&front[0..=fronti]) * solve_b(&back[backi..]);
                } else {
                    break;
                }
            }
        }
        arrangements
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let mut nums: Vec<u128> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();
    nums.push(0);
    nums.sort_unstable();
    nums.push(nums.last().unwrap() + 3);
    (solve_a(&nums).to_string(), solve_b(&nums).to_string())
}
