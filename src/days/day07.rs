use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Color<'a> = (&'a str, &'a str);

fn solve_a(inverse_rules: &HashMap<Color, HashMap<Color, usize>>) -> usize {
    let mut seen: HashSet<&Color> = HashSet::new();
    let mut queue: VecDeque<&Color> = VecDeque::new();
    queue.push_back(&("shiny", "gold"));
    while let Some(color) = queue.pop_front() {
        if seen.insert(color) {
            if let Some(containers) = inverse_rules.get(color) {
                queue.extend(containers.keys());
            }
        }
    }
    seen.len() - 1
}

fn solve_b(rules: &HashMap<Color, HashMap<Color, usize>>, color: &Color) -> usize {
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
    let mut rules: HashMap<Color, HashMap<Color, usize>> = HashMap::new();
    let mut inverse_rules: HashMap<Color, HashMap<Color, usize>> = HashMap::new();

    for line in lines {
        let mut words = line.split(' ');
        let container_color = (words.next().unwrap(), words.next().unwrap());
        words.next();
        words.next();

        let mut new_rules: HashMap<Color, usize> = HashMap::new();

        while let Some(num) = words.next() {
            if let Ok(num) = num.parse::<usize>() {
                let content_color = (
                    words.next().unwrap(),
                    words
                        .next()
                        .unwrap()
                        .trim_end_matches(',')
                        .trim_end_matches('.'),
                );
                words.next();

                new_rules.insert(content_color, num);
                inverse_rules
                    .entry(content_color)
                    .or_insert(HashMap::new())
                    .insert(container_color, num);
            } else {
                words.next();
                words.next();
            }
        }

        rules.insert(container_color, new_rules);
    }

    (
        solve_a(&inverse_rules).to_string(),
        (solve_b(&rules, &("shiny", "gold")) - 1).to_string(),
    )
}
