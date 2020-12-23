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

fn solve_a(mut cups: Vec<usize>) -> String {
    let l = cups.len();
    let mut current_index = 0;

    for _ in 0..100 {
        let current_label = cups[current_index];

        dbg!(&cups);
        dbg!(current_index, current_label);

        let cup1 = cups.remove(if current_index + 1 >= cups.len() {
            0
        } else {
            current_index + 1
        });
        let cup2 = cups.remove(if current_index + 1 >= cups.len() {
            0
        } else {
            current_index + 1
        });
        let cup3 = cups.remove(if current_index + 1 >= cups.len() {
            0
        } else {
            current_index + 1
        });

        dbg!(cup1, cup2, cup3);

        let dest_label: usize = **(&[
            mod1(current_label + l - 1, l),
            mod1(current_label + l - 2, l),
            mod1(current_label + l - 3, l),
            mod1(current_label + l - 4, l),
        ]
        .iter()
        .find(|c| **c != cup1 && **c != cup2 && **c != cup3)
        .unwrap());

        dbg!(dest_label);

        let (dest_index, _) = cups
            .iter()
            .enumerate()
            .find(|(_, c)| **c == dest_label)
            .unwrap();

        dbg!(dest_index);

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

    dbg!(&cups);

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

fn solve_b(lines: &[String]) -> usize {
    0
}

pub fn solve(lines: &[String]) -> Solution {
    let cups: Vec<usize> = lines
        .first()
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    (solve_a(cups).to_string(), solve_b(lines).to_string())
}
