use crate::common::Solution;

fn solve_a(lines: &[String]) -> i32 {
    let numbers: Vec<i32> = lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    for a in &numbers {
        for b in &numbers {
            if a != b && a + b == 2020 {
                return a * b;
            }
        }
    }
    unreachable!()
}

fn solve_b(lines: &[String]) -> i32 {
    let numbers: Vec<i32> = lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    for a in &numbers {
        for b in &numbers {
            for c in &numbers {
                if a != b && b != c && a != c && a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    unreachable!()
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b(lines).to_string())
}
