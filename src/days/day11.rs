use crate::common::Solution;

#[derive(Clone, PartialEq)]
enum Tile {
    Floor,
    Free,
    Occupied,
}

fn solve_a(map: Vec<Vec<Tile>>) -> usize {
    let h = map.len();
    let w = map[0].len();

    let mut buf1 = map;
    let mut buf2 = vec![vec![Tile::Floor; w]; h];

    let mut current = &mut buf1;
    let mut next = &mut buf2;

    let stable = loop {
        // println!(
        //     "{}\n",
        //     current
        //         .iter()
        //         .map(|row| row
        //             .iter()
        //             .map(|tile| match tile {
        //                 Tile::Floor => '.',
        //                 Tile::Free => 'L',
        //                 Tile::Occupied => '#',
        //             })
        //             .collect::<String>())
        //         .collect::<Vec<String>>()
        //         .join("\n")
        // );

        for r in 1..(h - 1) {
            for c in 1..(w - 1) {
                let num_neighbors = ((r - 1)..=(r + 1))
                    .flat_map(|nr| ((c - 1)..=(c + 1)).map(move |nc| (nr, nc)))
                    .filter(|(nr, nc)| (*nr, *nc) != (r, c))
                    .map(|(nr, nc)| &current[nr][nc])
                    .filter(|tile| **tile == Tile::Occupied)
                    .count();
                // println!("{}", num_neighbors);

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

                    Tile::Floor => Tile::Floor,
                };
            }
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

fn solve_b(map: Vec<Vec<Tile>>) -> usize {
    let h = map.len();
    let w = map[0].len();

    let mut buf1 = map;
    let mut buf2 = vec![vec![Tile::Floor; w]; h];

    let mut current = &mut buf1;
    let mut next = &mut buf2;

    let stable = loop {
        // println!(
        //     "{}\n",
        //     current
        //         .iter()
        //         .map(|row| row
        //             .iter()
        //             .map(|tile| match tile {
        //                 Tile::Floor => '.',
        //                 Tile::Free => 'L',
        //                 Tile::Occupied => '#',
        //             })
        //             .collect::<String>())
        //         .collect::<Vec<String>>()
        //         .join("\n")
        // );

        for r in 1..(h - 1) {
            for c in 1..(w - 1) {
                let mut num_neighbors = 0;
                for (dr, dc) in ((-1_isize)..=(1))
                    .flat_map(|dr| ((-1_isize)..=(1)).map(move |dc| (dr, dc)))
                    .filter(|(dr, dc)| (*dr, *dc) != (0, 0))
                {
                    for i in 1.. {
                        let nr = (r as isize + i * dr) as usize;
                        let nc = (c as isize + i * dc) as usize;
                        if nr == 0 || nc == 0 || nr >= h || nc >= w {
                            break;
                        } else {
                            match current[nr][nc] {
                                Tile::Floor => {}
                                Tile::Free => {
                                    break;
                                }
                                Tile::Occupied => {
                                    num_neighbors += 1;
                                    break;
                                }
                            }
                        }
                    }
                }

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

                    Tile::Floor => Tile::Floor,
                };
            }
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
            ['.']
                .iter()
                .copied()
                .chain(l.chars())
                .chain(['.'].iter().copied())
                .map(|c| match c {
                    '.' => Tile::Floor,
                    'L' => Tile::Free,
                    '#' => Tile::Occupied,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    map.insert(0, vec![Tile::Floor; map[0].len()]);
    map.push(vec![Tile::Floor; map[0].len()]);

    (solve_a(map.clone()).to_string(), solve_b(map).to_string())
}
