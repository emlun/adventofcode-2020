use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

fn vec_to_int(v: &Vec<bool>) -> u32 {
    v.iter()
        .fold(0, |i, next| (i << 1) | if *next { 1 } else { 0 })
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

    println!("{:?}", borders);
    for (id, bs) in &borders_int {
        println!("{} {:?}", id, bs);
    }
    println!();
    for (id, bs) in &borders_int_backward {
        println!("{} {:?}", id, bs);
    }

    // let compatibilities: HashSet<(&usize, &usize)> = borders_int
    //     .iter()
    //     .flat_map(|(id1, borders1)| {
    //         borders_int
    //             .iter()
    //             .filter(move |(id2, _)| id1 != *id2)
    //             .filter(move |(_, borders2)| borders1.iter().any(|b1| borders2.contains(&1)))
    //             .map(move |(id2, _)| (id1, id2))
    //     })
    //     .collect();

    // println!("{:?}", compatibilities);

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

    println!("{:?}", best_corners);

    best_corners.into_iter().product()
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

    println!("{:?}", tiles.len());
    // println!("{:?}", tiles);

    (solve_a(&tiles).to_string(), "".to_string())
}
