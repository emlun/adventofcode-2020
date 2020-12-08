use crate::common::Solution;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;

fn solve_a(program: &[(&str, i32)]) -> i32 {
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

fn solve_b(program: &[(&str, i32)]) -> i32 {
    for corruption_index in (0..program.len()).filter(|i| {
        let inst = program[*i].0;
        inst == "nop" || inst == "jmp"
    }) {
        let mut history: HashSet<i32> = HashSet::new();
        let mut acc: i32 = 0;
        let mut eip: i32 = 0;
        let (found, result) = loop {
            let eip_usize: usize = eip.try_into().unwrap();
            if eip_usize == program.len() {
                break (true, acc);
            } else if !history.insert(eip) {
                break (false, acc);
            } else {
                match program[eip_usize] {
                    ("nop", d) => {
                        if eip_usize == corruption_index {
                            eip += d;
                        } else {
                            eip += 1;
                        }
                    }
                    ("acc", d) => {
                        acc += d;
                        eip += 1;
                    }
                    ("jmp", d) => {
                        if eip_usize == corruption_index {
                            eip += 1;
                        } else {
                            eip += d;
                        }
                    }
                    _ => unreachable!(),
                };
            }
        };
        if found {
            return result;
        }
    }
    unreachable!()
}

pub fn solve(lines: &[String]) -> Solution {
    let program: Vec<(&str, i32)> = lines
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

    (solve_a(&program).to_string(), solve_b(&program).to_string())
}
