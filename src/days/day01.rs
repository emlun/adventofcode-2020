use crate::common::Solution;
use std::collections::HashSet;

const TARGET: i32 = 2020;

fn solve_a(numbers: &[i32], numset: &HashSet<i32>) -> i32 {
    for a in numbers {
        let b = TARGET - a;
        if numset.contains(&b) {
            return a * b;
        }
    }
    unreachable!();
}

fn solve_b(numbers: &[i32], numset: &HashSet<i32>) -> i32 {
    for (ia, a) in numbers.iter().enumerate() {
        for b in &numbers[(ia + 1)..] {
            let ab = a + b;
            if ab < TARGET {
                let c = TARGET - ab;
                if numset.contains(&c) {
                    return a * b * c;
                }
            } else {
                continue;
            }
        }
    }
    unreachable!();
}

pub fn solve(lines: &[String]) -> Solution {
    let mut numbers: Vec<i32> = lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    numbers.sort_unstable();
    let numset: HashSet<i32> = numbers.iter().copied().collect();

    (
        solve_a(&numbers, &numset).to_string(),
        solve_b(&numbers, &numset).to_string(),
    )
}
