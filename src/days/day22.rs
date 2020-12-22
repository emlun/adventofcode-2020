use crate::common::Solution;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hasher;

fn solve_a(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) -> usize {
    while !player1.is_empty() && !player2.is_empty() {
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

fn hash<T>(t: T) -> u64
where
    T: std::hash::Hash,
{
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn play_recursive(
    mut player1: VecDeque<usize>,
    mut player2: VecDeque<usize>,
) -> (bool, VecDeque<usize>) {
    let mut history: HashSet<(u64, u64)> = HashSet::new();
    while !player1.is_empty() && !player2.is_empty() {
        let game_hash = (hash(&player1), hash(&player2));
        if history.contains(&game_hash) {
            return (true, player1);
        } else {
            history.insert(game_hash);
        }

        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        let player1_won = if player1.len() >= card1 && player2.len() >= card2 {
            play_recursive(
                player1.iter().take(card1).copied().collect(),
                player2.iter().take(card2).copied().collect(),
            )
            .0
        } else {
            card1 > card2
        };

        if player1_won {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }

    if player2.is_empty() {
        (true, player1)
    } else {
        (false, player2)
    }
}

fn solve_b(player1: VecDeque<usize>, player2: VecDeque<usize>) -> usize {
    let (_, winner) = play_recursive(player1.clone(), player2.clone());
    let l = winner.len();
    winner
        .into_iter()
        .enumerate()
        .map(|(i, card)| (l - i) * card)
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut player1: VecDeque<usize> = VecDeque::new();

    let mut lines_it = lines.iter();
    assert_eq!(lines_it.next().unwrap(), "Player 1:");
    while let Some(line) = lines_it.next() {
        if line.is_empty() {
            assert_eq!(lines_it.next().unwrap(), "Player 2:");
            break;
        }
        player1.push_back(line.parse().unwrap());
    }
    let player2: VecDeque<usize> = lines_it.map(|line| line.parse().unwrap()).collect();

    (
        solve_a(player1.clone(), player2.clone()).to_string(),
        solve_b(player1, player2).to_string(),
    )
}
