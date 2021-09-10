use crate::common::Solution;
use std::collections::HashSet;

fn solve_a(lines: &[String]) -> HashSet<(i32, i32)> {
    let mut tiles: HashSet<(i32, i32)> = HashSet::new();

    for mut line in lines.iter().map(|s| s.as_str()) {
        let (mut hexx, mut hexy) = (0, 0);
        while !line.is_empty() {
            if let Some(remaining) = line.strip_prefix("nw") {
                hexx -= 1;
                hexy -= 1;
                line = remaining;
            } else if let Some(remaining) = line.strip_prefix("ne") {
                hexy -= 1;
                line = remaining;
            } else if let Some(remaining) = line.strip_prefix("sw") {
                hexy += 1;
                line = remaining;
            } else if let Some(remaining) = line.strip_prefix("se") {
                hexx += 1;
                hexy += 1;
                line = remaining;
            } else if let Some(remaining) = line.strip_prefix('w') {
                hexx -= 1;
                line = remaining;
            } else if let Some(remaining) = line.strip_prefix('e') {
                hexx += 1;
                line = remaining;
            } else {
                unreachable!();
            }
        }

        if tiles.contains(&(hexx, hexy)) {
            tiles.remove(&(hexx, hexy));
        } else {
            tiles.insert((hexx, hexy));
        }
    }

    tiles
}

fn solve_b(mut tiles: HashSet<(i32, i32)>) -> usize {
    const NUM_DAYS: usize = 100;

    for _ in 1..=NUM_DAYS {
        let affected_tiles: HashSet<(i32, i32)> = tiles
            .iter()
            .copied()
            .flat_map(|(hexx, hexy): (i32, i32)| {
                vec![
                    (hexx - 1, hexy - 1),
                    (hexx, hexy - 1),
                    (hexx - 1, hexy),
                    (hexx + 1, hexy),
                    (hexx, hexy),
                    (hexx, hexy + 1),
                    (hexx + 1, hexy + 1),
                ]
            })
            .collect();

        let new_tiles = affected_tiles
            .into_iter()
            .filter(|(hexx, hexy)| {
                let num_black_neighbors = [
                    (hexx - 1, hexy - 1),
                    (*hexx, hexy - 1),
                    (hexx - 1, *hexy),
                    (hexx + 1, *hexy),
                    (*hexx, hexy + 1),
                    (hexx + 1, hexy + 1),
                ]
                .iter()
                .copied()
                .filter(|nhexxy| tiles.contains(nhexxy))
                .count();

                if tiles.contains(&(*hexx, *hexy)) {
                    !(num_black_neighbors == 0 || num_black_neighbors > 2)
                } else {
                    num_black_neighbors == 2
                }
            })
            .collect();

        tiles = new_tiles;
    }

    tiles.len()
}

pub fn solve(lines: &[String]) -> Solution {
    let tiles = solve_a(lines);
    (tiles.len().to_string(), solve_b(tiles).to_string())
}
