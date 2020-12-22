use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn solve_a(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) -> usize {
    while !player1.is_empty() && !player2.is_empty() {
        dbg!(player1.len(), player2.len());
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }
    let winner = if player1.is_empty() { player2 } else { player1 };
    let l = winner.len();
    winner
        .into_iter()
        .enumerate()
        .map(|(i, card)| (l - i) * card)
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut player1: VecDeque<usize> = VecDeque::new();
    let mut player2: VecDeque<usize> = VecDeque::new();

    let mut lines_it = lines.iter();
    assert_eq!(lines_it.next().unwrap(), "Player 1:");
    while let Some(line) = lines_it.next() {
        if line.is_empty() {
            assert_eq!(lines_it.next().unwrap(), "Player 2:");
            break;
        }
        player1.push_back(line.parse().unwrap());
    }
    player2 = lines_it.map(|line| line.parse().unwrap()).collect();

    (solve_a(player1, player2).to_string(), "".to_string())
}
