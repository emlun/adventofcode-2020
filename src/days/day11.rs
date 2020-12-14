use crate::common::Solution;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Floor,
    Free,
    Occupied,
}

#[allow(unused)]
fn print_state(state: &[Vec<Tile>]) {
    println!(
        "{}\n",
        state
            .iter()
            .map(|row| row
                .iter()
                .map(|tile| match tile {
                    Tile::Floor => '.',
                    Tile::Free => 'L',
                    Tile::Occupied => '#',
                })
                .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn solve_a(map: Vec<Vec<Tile>>, nonfloors: &[(usize, usize)], drdc: &[(isize, isize)]) -> usize {
    let h = map.len();
    let w = map[0].len();

    let mut buf1 = map;
    let mut buf2 = vec![vec![Tile::Floor; w]; h];

    let mut current = &mut buf1;
    let mut next = &mut buf2;

    let stable = loop {
        for (r, c) in nonfloors.iter().copied() {
            let num_neighbors = drdc
                .iter()
                .map(|(dr, dc)| &current[(r as isize + dr) as usize][(c as isize + dc) as usize])
                .filter(|tile| **tile == Tile::Occupied)
                .count();

            next[r][c] = match current[r][c] {
                Tile::Free => {
                    if num_neighbors == 0 {
                        Tile::Occupied
                    } else {
                        Tile::Free
                    }
                }

                Tile::Occupied => {
                    if num_neighbors >= 4 {
                        Tile::Free
                    } else {
                        Tile::Occupied
                    }
                }

                Tile::Floor => unreachable!(),
            };
        }

        if current == next {
            break current;
        }

        std::mem::swap(&mut current, &mut next);
    };

    stable
        .iter_mut()
        .flatten()
        .filter(|tile| **tile == Tile::Occupied)
        .count()
}

fn solve_b(map: Vec<Vec<Tile>>, nonfloors: &[(usize, usize)], drdc: &[(isize, isize)]) -> usize {
    let h = map.len();
    let w = map[0].len();

    let neighbors: HashMap<(usize, usize), Vec<(usize, usize)>> = nonfloors
        .iter()
        .copied()
        .map(|(r, c)| {
            (
                (r, c),
                drdc.iter()
                    .flat_map(|(dr, dc)| {
                        (1..)
                            .map(|i| {
                                (
                                    (r as isize + i * dr) as usize,
                                    (c as isize + i * dc) as usize,
                                )
                            })
                            .take_while(|(nr, nc)| nr > &0 && nc > &0 && nr < &h && nc < &w)
                            .find(|(nr, nc)| map[*nr][*nc] != Tile::Floor)
                    })
                    .collect(),
            )
        })
        .collect();

    let mut buf1 = map;
    let mut buf2 = vec![vec![Tile::Floor; w]; h];

    let mut current = &mut buf1;
    let mut next = &mut buf2;

    let stable = loop {
        for (r, c) in nonfloors.iter().copied() {
            let num_neighbors = neighbors[&(r, c)]
                .iter()
                .filter(|(nr, nc)| current[*nr][*nc] == Tile::Occupied)
                .take(5)
                .count();

            next[r][c] = match current[r][c] {
                Tile::Free => {
                    if num_neighbors == 0 {
                        Tile::Occupied
                    } else {
                        Tile::Free
                    }
                }

                Tile::Occupied => {
                    if num_neighbors >= 5 {
                        Tile::Free
                    } else {
                        Tile::Occupied
                    }
                }

                Tile::Floor => unreachable!(),
            };
        }

        if current == next {
            break current;
        }

        std::mem::swap(&mut current, &mut next);
    };

    stable
        .iter_mut()
        .flatten()
        .filter(|tile| **tile == Tile::Occupied)
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut map: Vec<Vec<Tile>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            [Tile::Floor]
                .iter()
                .copied()
                .chain(l.chars().map(|c| match c {
                    '.' => Tile::Floor,
                    'L' => Tile::Free,
                    '#' => Tile::Occupied,
                    _ => unreachable!(),
                }))
                .chain([Tile::Floor].iter().copied())
                .collect()
        })
        .collect();
    map.insert(0, vec![Tile::Floor; map[0].len()]);
    map.push(vec![Tile::Floor; map[0].len()]);
    let map = map;

    let h = map.len();
    let w = map[0].len();
    let nonfloors: Vec<(usize, usize)> = (1..(h - 1))
        .flat_map(|r| (1..(w - 1)).map(move |c| (r, c)))
        .filter(|(r, c)| map[*r][*c] != Tile::Floor)
        .collect();

    let drdc: Vec<(isize, isize)> = ((-1)..=1)
        .flat_map(|dr| ((-1)..=1).map(move |dc| (dr, dc)))
        .filter(|(dr, dc)| (*dr, *dc) != (0, 0))
        .collect();

    (
        solve_a(map.clone(), &nonfloors, &drdc).to_string(),
        solve_b(map, &nonfloors, &drdc).to_string(),
    )
}
