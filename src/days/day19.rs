use crate::common::Solution;
use std::collections::BTreeMap;

#[derive(Debug)]
enum Rule<'slf> {
    Simple(&'slf str),
    Ref(Vec<Vec<usize>>),
}

fn match_rules<'a>(message: &'a str, rule: &Rule, rules: &BTreeMap<usize, Rule>) -> Vec<&'a str> {
    match rule {
        Rule::Simple(pat) => message
            .strip_prefix(pat)
            .map(|remaining| vec![remaining])
            .unwrap_or(vec![]),
        Rule::Ref(ors) => {
            let mut remainings = vec![];
            for seq in ors {
                let msgs = seq.iter().fold(vec![message], |msgs, subrule| {
                    msgs.into_iter()
                        .flat_map(|msg| match_rules(msg, &rules[subrule], rules).into_iter())
                        .collect()
                });
                remainings.extend(msgs);
            }
            remainings
        }
    }
}

fn matches<'a>(message: &'a str, rules: &BTreeMap<usize, Rule>) -> bool {
    match_rules(message, &rules[&0], rules).contains(&"")
}

fn solve_a(messages: &[&String], rules: &BTreeMap<usize, Rule>) -> usize {
    messages
        .iter()
        .filter(|message| matches(message, rules))
        .count()
}

fn solve_b(messages: &[&String], rules: BTreeMap<usize, Rule>) -> usize {
    let b_rules: BTreeMap<usize, Rule> = rules
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
        .filter(|message| matches(message, &b_rules))
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut input = lines.iter();
    let rules: BTreeMap<usize, Rule> = (&mut input)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split(':');
            let id = parts.next().unwrap().trim();
            let pattern = parts.next().unwrap().trim();
            (
                id.parse().unwrap(),
                if let Some(literal) = pattern.strip_prefix('"') {
                    Rule::Simple(literal.trim().strip_suffix('"').unwrap())
                } else {
                    Rule::Ref(
                        pattern
                            .split('|')
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
