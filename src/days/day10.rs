use crate::common::Solution;

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
    // Thanks @f00ale for this algorithm

    let mut paths_to = vec![0; nums.len()];
    paths_to[0] = 1;

    for i in 0..nums.len() - 1 {
        for j in (i + 1..nums.len()).take(3) {
            if nums[j] - nums[i] <= 3 {
                paths_to[j] += paths_to[i];
            }
        }
    }

    *paths_to.last().unwrap()
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
