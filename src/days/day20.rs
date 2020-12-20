use crate::common::Solution;
use std::collections::HashMap;

#[derive(Debug)]
struct Tile {
    id: usize,
    data: Vec<Vec<bool>>,
    borders: Vec<u32>,
    borders_flipped: Vec<u32>,
    matchables: Vec<u32>,
    matchables_flipped: Vec<u32>,
}

#[derive(Debug)]
struct TileRef {
    id: usize,
    flip: bool,
    rot: u8,
}

#[allow(unused)]
fn print_image(img: &[Vec<bool>]) {
    println!(
        "{}\n",
        img.iter()
            .map(|row| row
                .iter()
                .map(|tile| if *tile { '#' } else { '.' })
                .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn vec_to_int(v: &Vec<bool>) -> u32 {
    v.iter()
        .fold(0, |i, next| (i << 1) | if *next { 1 } else { 0 })
}

fn flip_vec<T>(v: &Vec<Vec<T>>, flip: bool) -> Vec<Vec<T>>
where
    T: Copy,
{
    if flip {
        v.clone()
            .into_iter()
            .map(|mut row| {
                row.reverse();
                row
            })
            .collect()
    } else {
        v.clone()
    }
}

fn rotate_vec<T>(v: &Vec<Vec<T>>, rot: u8) -> Vec<Vec<T>>
where
    T: Copy,
{
    match rot {
        0 => v.clone(),

        1 => (0..v.len())
            .map(|y| (0..v[y].len()).map(|x| v[v.len() - x - 1][y]).collect())
            .collect(),

        2 => (0..v.len())
            .map(|y| {
                (0..v[y].len())
                    .map(|x| v[v.len() - y - 1][v[x].len() - x - 1])
                    .collect()
            })
            .collect(),

        3 => (0..v.len())
            .map(|y| (0..v[y].len()).map(|x| v[x][v[x].len() - y - 1]).collect())
            .collect(),

        _ => unreachable!(),
    }
}

fn find_in_image(image: &Vec<Vec<bool>>, pattern: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    (0..image.len() - pattern.len())
        .flat_map(|y0| {
            (0..image[0].len() - pattern[0].len())
                .filter(move |x0| {
                    for py in 0..pattern.len() {
                        let y = y0 + py;
                        for px in 0..pattern[py].len() {
                            let x = x0 + px;
                            if pattern[py][px] && !image[y][x] {
                                return false;
                            }
                        }
                    }
                    return true;
                })
                .map(move |x0| (x0, y0))
        })
        .collect()
}

#[cfg(test)]
#[test]
fn test_rotate_vec() {
    let v = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];

    assert_eq!(rotate_vec(&v, 0), v);

    assert_eq!(
        rotate_vec(&v, 1),
        vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ],
    );

    assert_eq!(
        rotate_vec(&v, 2),
        vec![
            vec![16, 15, 14, 13],
            vec![12, 11, 10, 9],
            vec![8, 7, 6, 5],
            vec![4, 3, 2, 1],
        ],
    );

    assert_eq!(
        rotate_vec(&v, 3),
        vec![
            vec![4, 8, 12, 16],
            vec![3, 7, 11, 15],
            vec![2, 6, 10, 14],
            vec![1, 5, 9, 13],
        ],
    );
}

fn solve_a(tiles: &HashMap<usize, Vec<Vec<bool>>>) -> usize {
    let borders: HashMap<usize, Vec<Vec<bool>>> = tiles
        .iter()
        .map(|(id, tile)| {
            let top = tile[0].clone();
            let right: Vec<bool> = tile
                .iter()
                .map(|row| row.last().unwrap())
                .copied()
                .collect();
            let mut bottom = tile.last().unwrap().clone();
            bottom.reverse();
            let mut left: Vec<bool> = tile
                .iter()
                .map(|row| row.first().unwrap())
                .copied()
                .collect();
            left.reverse();
            (*id, vec![top, right, bottom, left])
        })
        .collect();

    let borders_int: HashMap<usize, Vec<u32>> = borders
        .iter()
        .map(|(id, bs)| (*id, bs.iter().map(vec_to_int).collect()))
        .collect();

    let borders_int_backward: HashMap<usize, Vec<u32>> = borders
        .iter()
        .map(|(id, bs)| {
            (
                *id,
                bs.clone()
                    .into_iter()
                    .map(|mut b| {
                        b.reverse();
                        vec_to_int(&b)
                    })
                    .collect(),
            )
        })
        .collect();

    let best_corners: Vec<&usize> = borders
        .keys()
        .filter(|id1| {
            let num_partners = borders_int[id1]
                .iter()
                .filter(|b1| {
                    borders_int
                        .iter()
                        .filter(|(id2, _)| id2 != id1)
                        .any(|(_, borders2)| borders2.contains(b1))
                        || borders_int_backward
                            .iter()
                            .filter(|(id2, _)| id2 != id1)
                            .any(|(_, borders2)| borders2.contains(b1))
                })
                .count();
            num_partners == 2
        })
        .collect();

    best_corners.into_iter().product()
}

fn solve_b(tiles: HashMap<usize, Vec<Vec<bool>>>) -> usize {
    let tiles: HashMap<usize, Tile> = tiles
        .into_iter()
        .map(|(id, data)| {
            let mut top = data[0].clone();
            let mut right: Vec<bool> = data
                .iter()
                .map(|row| row.last().unwrap())
                .copied()
                .collect();
            let mut bottom = data.last().unwrap().clone();
            bottom.reverse();
            let mut left: Vec<bool> = data
                .iter()
                .map(|row| row.first().unwrap())
                .copied()
                .collect();
            left.reverse();

            let borders = vec![
                vec_to_int(&top),
                vec_to_int(&right),
                vec_to_int(&bottom),
                vec_to_int(&left),
            ];

            let matchables_flipped = vec![
                vec_to_int(&top),
                vec_to_int(&left),
                vec_to_int(&bottom),
                vec_to_int(&right),
            ];

            top.reverse();
            right.reverse();
            bottom.reverse();
            left.reverse();

            let matchables = vec![
                vec_to_int(&top),
                vec_to_int(&right),
                vec_to_int(&bottom),
                vec_to_int(&left),
            ];

            let borders_flipped = vec![
                vec_to_int(&top),
                vec_to_int(&left),
                vec_to_int(&bottom),
                vec_to_int(&right),
            ];

            (
                id,
                Tile {
                    id,
                    data,
                    borders,
                    borders_flipped,
                    matchables,
                    matchables_flipped,
                },
            )
        })
        .collect();

    let partners: HashMap<usize, Vec<Vec<usize>>> = tiles
        .keys()
        .map(|id1| {
            (
                *id1,
                tiles[id1]
                    .borders
                    .iter()
                    .map(|b1| {
                        tiles
                            .keys()
                            .filter(|id2| *id2 != id1)
                            .filter(|id2| {
                                tiles[id2].matchables.contains(b1)
                                    || tiles[id2].matchables_flipped.contains(b1)
                            })
                            .copied()
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect();

    let partners_flipped: HashMap<usize, Vec<Vec<usize>>> = tiles
        .keys()
        .map(|id1| {
            (
                *id1,
                tiles[id1]
                    .borders_flipped
                    .iter()
                    .map(|b1| {
                        tiles
                            .keys()
                            .filter(|id2| *id2 != id1)
                            .filter(|id2| {
                                tiles[id2].matchables.contains(b1)
                                    || tiles[id2].matchables_flipped.contains(b1)
                            })
                            .copied()
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect();

    let mut corners: Vec<&usize> = tiles
        .keys()
        .filter(|id1| partners[id1].iter().filter(|dir| !dir.is_empty()).count() == 2)
        .collect();

    let first_tile_id = *corners.pop().unwrap();
    let first_tile_rot = match (
        !partners[&first_tile_id][0].is_empty(),
        !partners[&first_tile_id][1].is_empty(),
        !partners[&first_tile_id][2].is_empty(),
        !partners[&first_tile_id][3].is_empty(),
    ) {
        (false, true, true, false) => 0,
        (true, true, false, false) => 1,
        (true, false, false, true) => 2,
        (false, false, true, true) => 3,
        _ => unreachable!(),
    };

    let mut assembled_image_refs: Vec<Vec<TileRef>> = vec![vec![TileRef {
        id: first_tile_id,
        flip: false,
        rot: first_tile_rot,
    }]];

    let (mut x, mut y) = (1, 0);
    while assembled_image_refs
        .iter()
        .map(|row| row.len())
        .sum::<usize>()
        < tiles.len()
    {
        let (anchor_tile, source_dir, target_dir) = if x == 0 {
            (&assembled_image_refs[y - 1][x], 2, 0)
        } else {
            (&assembled_image_refs[y][x - 1], 1, 3)
        };

        let possible_partners = if anchor_tile.flip {
            &partners_flipped[&anchor_tile.id]
        } else {
            &partners[&anchor_tile.id]
        };

        assert_eq!(
            possible_partners[((source_dir + 4 - anchor_tile.rot) % 4) as usize].len(),
            1
        );

        let partner_tile =
            &tiles[&possible_partners[((source_dir + 4 - anchor_tile.rot) % 4) as usize][0]];

        let anchor = if anchor_tile.flip {
            &tiles[&anchor_tile.id].borders_flipped
        } else {
            &tiles[&anchor_tile.id].borders
        }[((source_dir + 4 - anchor_tile.rot) % 4) as usize];

        let flip = !partner_tile.matchables.contains(&anchor);
        let rot = (if flip {
            &partner_tile.matchables_flipped
        } else {
            &partner_tile.matchables
        })
        .iter()
        .enumerate()
        .find(|(_, matchable)| **matchable == anchor)
        .map(|(i, _)| (target_dir + 4 - i as u8) % 4)
        .unwrap();

        assembled_image_refs[y].push(TileRef {
            id: partner_tile.id,
            flip,
            rot,
        });

        if (if flip { &partners_flipped } else { &partners })[&partner_tile.id]
            [((1 + 4 - rot) % 4) as usize]
            .is_empty()
        {
            y += 1;
            x = 0;
            assembled_image_refs.push(vec![]);
        } else {
            x += 1;
        }
    }
    assembled_image_refs.remove(assembled_image_refs.len() - 1);

    let mut assembled_image: Vec<Vec<bool>> = vec![];
    for ref_row in assembled_image_refs {
        let y0 = assembled_image.len();
        assembled_image.resize(
            assembled_image.len() + tiles[&ref_row[0].id].data.len() - 2,
            vec![],
        );
        for tile_ref in ref_row {
            let data = rotate_vec(
                &flip_vec(&tiles[&tile_ref.id].data, tile_ref.flip),
                tile_ref.rot,
            );
            for (dy, data_row) in data.iter().skip(1).take(data.len() - 2).enumerate() {
                assembled_image[y0 + dy].extend(&data_row[1..data_row.len() - 1]);
            }
        }
    }

    let sea_monster: Vec<Vec<bool>> = r"
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
"
    .lines()
    .filter(|l| !l.is_empty())
    .map(|line| line.chars().map(|c| c == '#').collect())
    .collect();

    let mut rot = 0;
    let sea_monsters = loop {
        let sea_monsters = find_in_image(&assembled_image, &sea_monster);
        if !sea_monsters.is_empty() {
            break sea_monsters;
        }
        rot += 1;
        assembled_image = rotate_vec(&assembled_image, 1);
        if rot == 4 {
            assembled_image = flip_vec(&assembled_image, true);
        } else if rot == 8 {
            panic!("Found no sea monsters!");
        }
    };

    for (x0, y0) in sea_monsters {
        for py in 0..sea_monster.len() {
            for px in 0..sea_monster[py].len() {
                if sea_monster[py][px] {
                    assembled_image[y0 + py][x0 + px] = false;
                }
            }
        }
    }

    assembled_image.iter().flatten().filter(|b| **b).count()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut tiles: HashMap<usize, Vec<Vec<bool>>> = HashMap::new();

    let mut lines_it = lines.iter();
    while let Some(tile_id) = lines_it.next() {
        let tile_id = tile_id
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse()
            .unwrap();
        let mut tile: Vec<Vec<bool>> = vec![];
        while let Some(l) = lines_it.next() {
            if l.is_empty() {
                break;
            } else {
                tile.push(
                    l.chars()
                        .map(|c| match c {
                            '.' => false,
                            '#' => true,
                            _ => unreachable!(),
                        })
                        .collect::<Vec<bool>>(),
                );
            }
        }
        tiles.insert(tile_id, tile);
    }

    (solve_a(&tiles).to_string(), solve_b(tiles).to_string())
}
