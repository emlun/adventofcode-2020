use crate::common::Solution;
use std::collections::HashMap;

#[derive(Debug)]
enum Rule<'slf> {
    Simple(&'slf str),
    Ref(Vec<Vec<usize>>),
}

fn matches<'a>(
    message: &'a str,
    rule: &Rule,
    rules: &HashMap<usize, Rule>,
) -> Option<Vec<&'a str>> {
    match rule {
        Rule::Simple(pat) => message.strip_prefix(pat).map(|remaining| vec![remaining]),
        Rule::Ref(ors) => {
            let mut remainings = vec![];
            for seq in ors {
                let mut msgs = vec![message];
                for subrule in seq {
                    msgs = msgs
                        .into_iter()
                        .flat_map(|msg| matches(msg, &rules[subrule], rules).into_iter().flatten())
                        .collect();
                }
                remainings.extend(msgs);
            }
            if remainings.len() > 0 {
                Some(remainings)
            } else {
                None
            }
        }
    }
}

fn solve_a(messages: &[&String], rules: &HashMap<usize, Rule>) -> usize {
    messages
        .iter()
        .filter(|message| {
            matches(message, &rules[&0], rules)
                .map(|v| v.iter().any(|s| s == &""))
                .unwrap_or(false)
        })
        .count()
}

fn solve_b(messages: &[&String], rules: HashMap<usize, Rule>) -> usize {
    let b_rules: HashMap<usize, Rule> = rules
        .into_iter()
        .map(|(id, rule)| match (id, rule) {
            (8, Rule::Ref(v)) if v == vec![vec![42]] => (8, Rule::Ref(vec![vec![42], vec![42, 8]])),
            (11, Rule::Ref(v)) if v == vec![vec![42, 31]] => {
                (11, Rule::Ref(vec![vec![42, 31], vec![42, 11, 31]]))
            }
            (id, rule) => (id, rule),
        })
        .collect();
    messages
        .iter()
        .filter(|message| {
            matches(message, &b_rules[&0], &b_rules)
                .map(|v| v.iter().any(|s| s == &""))
                .unwrap_or(false)
        })
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
    (
        solve_a(&messages, &rules).to_string(),
        solve_b(&messages, rules).to_string(),
    )
}
