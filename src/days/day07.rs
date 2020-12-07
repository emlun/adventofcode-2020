use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn solve_a(inverse_rules: &HashMap<String, HashMap<String, usize>>) -> usize {
    let mut seen: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("shiny gold");
    while let Some(color) = queue.pop_front() {
        seen.insert(color);
        if let Some(containers) = inverse_rules.get(color) {
            queue.extend(containers.keys().map(|s| s.as_str()));
        }
    }
    seen.len() - 1
}

fn solve_b(rules: &HashMap<String, HashMap<String, usize>>, color: &str) -> usize {
    1 + if let Some(contents) = rules.get(color) {
        contents
            .iter()
            .map(|(content_color, num)| num * solve_b(rules, content_color))
            .sum()
    } else {
        0
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let mut rules: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut inverse_rules: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for line in lines {
        let mut words = line.split(' ');
        let container_color = format!("{} {}", words.next().unwrap(), words.next().unwrap());
        words.next();
        words.next();

        let mut new_rules: HashMap<String, usize> = HashMap::new();

        while let Some(num) = words.next() {
            if let Ok(num) = num.parse::<usize>() {
                let content_color = format!("{} {}", words.next().unwrap(), words.next().unwrap())
                    .trim_end_matches(',')
                    .trim_end_matches('.')
                    .to_string();
                words.next();

                new_rules.insert(content_color.clone(), num);
                inverse_rules
                    .entry(content_color)
                    .or_insert(HashMap::new())
                    .insert(container_color.clone(), num);
            } else {
                words.next();
                words.next();
            }
        }

        rules.insert(container_color, new_rules);
    }

    (
        solve_a(&inverse_rules).to_string(),
        (solve_b(&rules, "shiny gold") - 1).to_string(),
    )
}
