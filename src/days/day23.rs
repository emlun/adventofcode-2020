use crate::common::Solution;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hasher;

fn mod1(a: usize, modulus: usize) -> usize {
    if a > modulus {
        (a % (modulus + 1)) + 1
    } else {
        a
    }
}

fn simulate(mut cups: VecDeque<usize>, moves: usize) -> VecDeque<usize> {
    let l = cups.len();
    let mut current_index = 0;

    for _ in 0..moves {
        // dbg!(mv);

        let current_label = cups[current_index];

        // dbg!(&cups);
        // dbg!(current_index, current_label);

        let cup1 = cups
            .remove(if current_index + 1 >= cups.len() {
                0
            } else {
                current_index + 1
            })
            .unwrap();
        let cup2 = cups
            .remove(if current_index + 1 >= cups.len() {
                0
            } else {
                current_index + 1
            })
            .unwrap();
        let cup3 = cups
            .remove(if current_index + 1 >= cups.len() {
                0
            } else {
                current_index + 1
            })
            .unwrap();

        // dbg!(cup1, cup2, cup3);

        let dest_label: usize = **(&[
            mod1(current_label + l - 1, l),
            mod1(current_label + l - 2, l),
            mod1(current_label + l - 3, l),
            mod1(current_label + l - 4, l),
        ]
        .iter()
        .find(|c| **c != cup1 && **c != cup2 && **c != cup3)
        .unwrap());

        // dbg!(dest_label);

        let (dest_index, _) = cups
            .iter()
            .enumerate()
            .find(|(_, c)| **c == dest_label)
            .unwrap();

        // dbg!(dest_index);

        cups.insert(dest_index + 1, cup1);
        cups.insert(dest_index + 2, cup2);
        cups.insert(dest_index + 3, cup3);

        current_index = cups
            .iter()
            .enumerate()
            .find(|(_, c)| **c == current_label)
            .unwrap()
            .0;

        current_index = (current_index + 1) % l;
    }

    // dbg!(&cups);

    cups
}

fn solve_a(cups: VecDeque<usize>) -> String {
    let cups = simulate(cups, 100);

    let labels = cups
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");
    let mut labels_rotated = labels.split('1');
    let left = labels_rotated.next().unwrap();
    let right = labels_rotated.next().unwrap();
    format!("{}{}", right, left)
}

fn solve_b(cups: VecDeque<usize>) -> usize {
    let l = cups.len();
    let cups = simulate(
        cups.into_iter().chain(l + 1..=1_000_000).collect(),
        10_000_000,
    );
    let one_index = cups.iter().enumerate().find(|(_, c)| **c == 1).unwrap().0;
    cups[(one_index + 1) % cups.len()] * cups[(one_index + 2) % cups.len()]
}

pub fn solve(lines: &[String]) -> Solution {
    let cups: VecDeque<usize> = lines
        .first()
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    (solve_a(cups.clone()).to_string(), solve_b(cups).to_string())
}
