use crate::common::Solution;

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

fn solve_b(nums: &[usize]) -> usize {
    if nums.is_empty() {
        0
    } else if nums.len() == 1 {
        1
    } else {
        let midi = nums.len() / 2;

        let front = &nums[0..midi];
        let back = &nums[midi..];

        let mut arrangements = 0;

        for fronti in 0..std::cmp::min(3, front.len()) {
            for backi in 0..std::cmp::min(3, back.len()) {
                if back[backi] - front[front.len() - fronti - 1] <= 3 {
                    arrangements +=
                        solve_b(&front[0..=front.len() - fronti - 1]) * solve_b(&back[backi..]);
                }
            }
        }
        arrangements
    }
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
    (a_solution.to_string(), solve_b(&nums).to_string())
}
