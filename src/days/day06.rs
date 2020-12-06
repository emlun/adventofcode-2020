use crate::common::Solution;
use std::collections::HashSet;

fn solve_a(groups: &[HashSet<char>]) -> usize {
    groups.iter().map(|g| g.len()).sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut lines: Vec<&String> = lines.iter().skip_while(|l| l.is_empty()).collect();
    while lines[lines.len() - 1].is_empty() {
        lines.pop();
    }
    let lines = lines;

    let groups_a: Vec<HashSet<char>> =
        lines.iter().fold(vec![HashSet::new()], |mut result, line| {
            if line.is_empty() {
                result.push(HashSet::new());
            } else {
                result.last_mut().unwrap().extend(line.chars());
            }
            result
        });

    let groups_b: Vec<HashSet<char>> =
        lines
            .iter()
            .fold(vec![('a'..='z').collect()], |mut result, line| {
                if line.is_empty() {
                    result.push(('a'..='z').collect());
                } else {
                    let person_ans: HashSet<char> = line.chars().collect();
                    let last = result.last_mut().unwrap();
                    last.retain(|c| person_ans.contains(c));
                }
                result
            });

    (
        solve_a(&groups_a).to_string(),
        solve_a(&groups_b).to_string(),
    )
}
