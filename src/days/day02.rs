use crate::common::Solution;
use std::collections::HashSet;

const TARGET: i32 = 2020;

fn solve_a(entries: &[(usize, usize, char, &str)]) -> usize {
    entries
        .iter()
        .filter(|(min, max, passchar, password)| {
            let charcount = password.chars().filter(|c| c == passchar).count();
            charcount >= *min && charcount <= *max
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut entries: Vec<(usize, usize, char, &str)> = lines
        .iter()
        .map(|line| {
            let mut parts1 = line.split(": ");
            let policy = parts1.next().unwrap();
            let password = parts1.next().unwrap();
            let mut parts2 = policy.split(" ");
            let minmax = parts2.next().unwrap();
            let passchar = parts2.next().unwrap();
            let mut parts3 = minmax.split("-");
            (
                parts3.next().unwrap().parse().unwrap(),
                parts3.next().unwrap().parse().unwrap(),
                passchar.chars().nth(0).unwrap(),
                password,
            )
        })
        .collect();

    (solve_a(&entries).to_string(), "".to_string())
}
