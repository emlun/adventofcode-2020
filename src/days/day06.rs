use crate::common::Solution;
use std::collections::HashSet;

fn solve_a(groups: &[Vec<HashSet<char>>]) -> usize {
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(HashSet::<char>::new(), |mut union, next| {
                    union.extend(next);
                    union
                })
                .len()
        })
        .sum()
}

fn solve_b(groups: Vec<Vec<HashSet<char>>>) -> usize {
    groups
        .into_iter()
        .map(|mut group| {
            let init = group.pop().unwrap();
            group
                .into_iter()
                .fold(init, |mut isct, next| {
                    isct.retain(|c| next.contains(c));
                    isct
                })
                .len()
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let lines = {
        let mut lines: &[String] = lines;
        while lines[lines.len() - 1].is_empty() {
            lines = &lines[..lines.len() - 1];
        }
        lines
    };

    let groups: Vec<Vec<HashSet<char>>> = lines.iter().fold(vec![vec![]], |mut result, line| {
        if line.is_empty() {
            result.push(vec![]);
        } else {
            result.last_mut().unwrap().push(line.chars().collect());
        }
        result
    });

    (solve_a(&groups).to_string(), solve_b(groups).to_string())
}
