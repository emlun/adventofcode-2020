use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
enum Rule<'slf> {
    Simple(&'slf str),
    Ref(Vec<Vec<usize>>),
}

fn matches<'a>(message: &'a str, rule: &Rule, rules: &HashMap<usize, Rule>) -> Option<&'a str> {
    match rule {
        Rule::Simple(pat) => message.strip_prefix(pat),
        Rule::Ref(ors) => {
            'outer: for seq in ors {
                let mut msg = message;
                for subrule in seq {
                    if let Some(remaining) = matches(msg, &rules[subrule], rules) {
                        msg = remaining;
                    } else {
                        continue 'outer;
                    }
                }
                return Some(msg);
            }
            None
        }
    }
}

fn solve_a(messages: &[&String], rules: &HashMap<usize, Rule>) -> usize {
    messages
        .iter()
        .filter(|message| matches(message, &rules[&0], rules) == Some(""))
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut input = lines.iter();
    let rules: HashMap<usize, Rule> = (&mut input)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split(":");
            let id = parts.next().unwrap().trim();
            let pattern = parts.next().unwrap().trim();
            (
                id.parse().unwrap(),
                if let Some(literal) = pattern.strip_prefix('"') {
                    Rule::Simple(literal.trim().strip_suffix('"').unwrap())
                } else {
                    Rule::Ref(
                        pattern
                            .split("|")
                            .map(|seq| seq.trim())
                            .map(|seq| seq.split(' ').map(|s| s.parse().unwrap()).collect())
                            .collect(),
                    )
                },
            )
        })
        .collect();
    let messages: Vec<&String> = input.skip_while(|l| l.is_empty()).collect();
    (solve_a(&messages, &rules).to_string(), "".to_string())
}
