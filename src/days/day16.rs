use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

fn solve_a(rules: &HashMap<&str, Vec<RangeInclusive<u16>>>, other_tickets: &[Vec<u16>]) -> u16 {
    other_tickets
        .iter()
        .flat_map(|t| t.iter())
        .filter(|n| {
            !rules
                .values()
                .flatten()
                .any(|allowed: &RangeInclusive<u16>| allowed.contains(*n))
        })
        .sum()
}

fn discard_invalid(
    rules: &HashMap<&str, Vec<RangeInclusive<u16>>>,
    tickets: &[Vec<u16>],
) -> Vec<Vec<u16>> {
    tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|n| {
                rules
                    .values()
                    .flatten()
                    .any(|allowed: &RangeInclusive<u16>| allowed.contains(n))
            })
        })
        .cloned()
        .collect()
}

fn solve_b(
    rules: &HashMap<&str, Vec<RangeInclusive<u16>>>,
    your_ticket: &[u16],
    other_tickets: &[Vec<u16>],
) -> u64 {
    let valid_other_tickets = discard_invalid(rules, other_tickets);

    let field_keys: HashSet<usize> =
        (0..valid_other_tickets.iter().map(|t| t.len()).max().unwrap()).collect();

    let mut field_mapping: HashMap<&usize, HashSet<&&str>> = field_keys
        .iter()
        .map(|i| {
            let values_in_this_field: HashSet<u16> = valid_other_tickets
                .iter()
                .flat_map(|ticket| ticket.get(*i))
                .copied()
                .collect();

            let possible_fields = rules
                .iter()
                .filter(|(_, ranges)| {
                    values_in_this_field
                        .iter()
                        .all(|v| ranges.iter().any(|range| range.contains(v)))
                })
                .map(|(k, _)| k)
                .collect();

            (i, possible_fields)
        })
        .collect();

    while field_mapping.iter().any(|(_, v)| v.len() > 1) {
        for field_key in &field_keys {
            if field_mapping[field_key].len() > 1 {
                let disallowed_fields: HashSet<&&str> = field_mapping
                    .iter()
                    .filter(|(k, v)| **k != field_key && v.len() == 1)
                    .flat_map(|(_, v)| v.iter())
                    .copied()
                    .collect();

                field_mapping
                    .get_mut(field_key)
                    .unwrap()
                    .retain(|k| !disallowed_fields.contains(k));
            }
        }
    }

    field_mapping
        .iter()
        .filter(|(_, v)| v.iter().next().unwrap().starts_with("departure"))
        .map(|(k, _)| u64::from(your_ticket[**k]))
        .product()
}

pub fn solve(lines: &[String]) -> Solution {
    enum ParsePhase {
        Prelude,
        YourTicket,
        OtherTickets,
    }
    let mut phase = ParsePhase::Prelude;

    let mut rules: HashMap<&str, Vec<RangeInclusive<u16>>> = HashMap::new();
    let mut your_ticket: Vec<u16> = vec![];
    let mut other_tickets: Vec<Vec<u16>> = vec![];

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
        solve_a(&rules, &other_tickets).to_string(),
        solve_b(&rules, &your_ticket, &other_tickets).to_string(),
    )
}
