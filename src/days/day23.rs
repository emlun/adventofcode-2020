use crate::common::Solution;

fn simulate(start_order: &Vec<usize>, l: usize, moves: usize) -> Vec<usize> {
    let mut links: Vec<usize> = (0..start_order.len())
        .into_iter()
        .map(|i| {
            start_order
                .iter()
                .enumerate()
                .find(|(_, s)| **s == i)
                .map(|(j, _)| {
                    if j + 1 >= start_order.len() && start_order.len() < l {
                        j + 1
                    } else {
                        start_order[(j + 1) % l]
                    }
                })
                .unwrap()
        })
        .chain((start_order.len() + 1)..l)
        .collect();
    if l > start_order.len() {
        links.push(start_order[0]);
    }
    let mut current_cup = start_order[0];

    for _ in 0..moves {
        let cup1 = links[current_cup];
        let cup2 = links[cup1];
        let cup3 = links[cup2];

        let destination: usize = (1..=4)
            .map(|i| (current_cup + l - i) % l)
            .find(|i| *i != cup1 && *i != cup2 && *i != cup3)
            .unwrap();

        links[current_cup] = links[cup3];
        links[cup3] = links[destination];
        links[destination] = cup1;

        current_cup = links[current_cup];
    }

    links
}

fn solve_a(cups: &Vec<usize>) -> String {
    let cups = simulate(cups, cups.len(), 100);
    let mut labels = vec![];
    let mut ii = 0;
    for _ in 1..cups.len() {
        labels.push(cups[ii] + 1);
        ii = cups[ii];
    }
    labels
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn solve_b(cups: &Vec<usize>) -> usize {
    let cups = simulate(cups, 1_000_000, 10_000_000);
    (cups[0] + 1) * (cups[cups[0]] + 1)
}

pub fn solve(lines: &[String]) -> Solution {
    let cups: Vec<usize> = lines
        .first()
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .map(|i: usize| i - 1)
        .collect();
    (solve_a(&cups).to_string(), solve_b(&cups).to_string())
}
