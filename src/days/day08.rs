use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::convert::TryInto;

type Color<'a> = (&'a str, &'a str);

fn solve_a(program: &[String]) -> i32 {
    let program: Vec<(&str, i32)> = program
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut words = l.split(' ');
            (
                words.next().unwrap(),
                words.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let mut acc = 0;
    let mut eip = 0;
    let mut history: HashSet<i32> = HashSet::new();
    loop {
        if !history.insert(eip.try_into().unwrap()) {
            break acc;
        }

        match program[usize::try_from(eip).unwrap()] {
            ("nop", _) => {}
            ("acc", d) => {
                acc += d;
            }
            ("jmp", d) => {
                eip += d - 1;
            }
            _ => unreachable!(),
        };
        eip += 1;
    }
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), "".to_string())
}
