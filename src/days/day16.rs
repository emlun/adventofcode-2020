use crate::common::Solution;
use std::collections::HashMap;
use std::ops::RangeInclusive;

fn solve_a(
    rules: &HashMap<&str, Vec<RangeInclusive<usize>>>,
    your_ticket: &Vec<usize>,
    other_tickets: &Vec<Vec<usize>>,
) -> usize {
    other_tickets
        .iter()
        .flat_map(|t| t.iter())
        .filter(|n| {
            dbg!(n);
            !rules
                .values()
                .flatten()
                .any(|allowed: &RangeInclusive<usize>| allowed.contains(*n))
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    enum ParsePhase {
        Prelude,
        YourTicket,
        OtherTickets,
    }
    let mut phase = ParsePhase::Prelude;

    let mut rules: HashMap<&str, Vec<RangeInclusive<usize>>> = HashMap::new();
    let mut your_ticket: Vec<usize> = vec![];
    let mut other_tickets: Vec<Vec<usize>> = vec![];

    for line in lines {
        if line.is_empty() {
        } else if line == "your ticket:" {
            phase = ParsePhase::YourTicket;
        } else if line == "nearby tickets:" {
            phase = ParsePhase::OtherTickets;
        } else {
            match phase {
                ParsePhase::Prelude => {
                    let mut sides = line.split(": ");
                    let lhs = sides.next().unwrap();
                    dbg!(&lhs);
                    let rhs = sides.next().unwrap();
                    rules.insert(
                        lhs,
                        rhs.split(" or ")
                            .map(|s| {
                                let mut parts = s.split('-');
                                parts.next().unwrap().parse().unwrap()
                                    ..=parts.next().unwrap().parse().unwrap()
                            })
                            .collect(),
                    );
                }

                ParsePhase::YourTicket => {
                    your_ticket = line.split(',').map(|s| s.parse().unwrap()).collect();
                }

                ParsePhase::OtherTickets => {
                    other_tickets.push(line.split(',').map(|s| s.parse().unwrap()).collect());
                }
            }
        }
    }

    (
        solve_a(&rules, &your_ticket, &other_tickets).to_string(),
        "".to_string(),
    )
}
