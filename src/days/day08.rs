use crate::common::Solution;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;

type Word = i32;
enum Instruction {
    Nop(Word),
    Acc(Word),
    Jmp(Word),
}
type Program = Vec<Instruction>;
use Instruction::*;

fn parse(lines: &[String]) -> Vec<Instruction> {
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut words = l.split(' ');
            let instr = words.next().unwrap();
            let arg = words.next().unwrap().parse().unwrap();
            match instr {
                "nop" => Nop(arg),
                "acc" => Acc(arg),
                "jmp" => Jmp(arg),
                _ => unimplemented!(),
            }
        })
        .collect()
}

fn solve_a(program: &[Instruction]) -> Word {
    let mut acc = 0;
    let mut eip = 0;
    let mut history: HashSet<Word> = HashSet::new();

    loop {
        if !history.insert(eip) {
            break acc;
        }

        match program[usize::try_from(eip).unwrap()] {
            Nop(_) => {}
            Acc(d) => {
                acc += d;
            }
            Jmp(d) => {
                eip += d - 1;
            }
        };
        eip += 1;
    }
}

fn solve_b(program: &[Instruction]) -> Word {
    for corruption_index in (0..program.len()).filter(|i| matches!(program[*i], Nop(_) | Jmp(_))) {
        let mut history: HashSet<Word> = HashSet::new();
        let mut acc: Word = 0;
        let mut eip: Word = 0;

        loop {
            let eip_usize: usize = eip.try_into().unwrap();
            if eip_usize == program.len() {
                return acc;
            } else if !history.insert(eip) {
                break;
            } else {
                match program[eip_usize] {
                    Nop(d) => {
                        if eip_usize == corruption_index {
                            eip += d;
                        } else {
                            eip += 1;
                        }
                    }
                    Acc(d) => {
                        acc += d;
                        eip += 1;
                    }
                    Jmp(d) => {
                        if eip_usize == corruption_index {
                            eip += 1;
                        } else {
                            eip += d;
                        }
                    }
                };
            }
        }
    }
    unreachable!()
}

pub fn solve(lines: &[String]) -> Solution {
    let program: Program = parse(lines);
    (solve_a(&program).to_string(), solve_b(&program).to_string())
}
