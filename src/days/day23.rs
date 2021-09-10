use crate::common::Solution;

fn simulate(start_order: &Vec<usize>, l: usize, moves: usize) -> Vec<usize> {
    let mut links: Vec<usize> =
        (0..start_order.len()).fold(vec![0; start_order.len()], |mut links, i| {
            links[start_order[i]] = start_order[(i + 1) % start_order.len()];
            links
        });
    links.extend((start_order.len() + 1)..=l);
    if l > start_order.len() {
        links[*start_order.last().unwrap()] = start_order.len();
        links[l - 1] = start_order[0];
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

fn solve_a(start_order: &Vec<usize>) -> String {
    let links = simulate(start_order, start_order.len(), 100);
    std::iter::successors(Some(links[0]), |i| Some(links[*i]))
        .take(links.len() - 1)
        .map(|i| (i + 1).to_string())
        .collect::<String>()
}

fn solve_b(start_order: &Vec<usize>) -> usize {
    let links = simulate(start_order, 1_000_000, 10_000_000);
    (links[0] + 1) * (links[links[0]] + 1)
}

pub fn solve(lines: &[String]) -> Solution {
    let init: Vec<usize> = lines
        .first()
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .map(|i: usize| i - 1)
        .collect();
    (solve_a(&init).to_string(), solve_b(&init).to_string())
}
