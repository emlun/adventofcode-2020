use crate::common::Solution;

fn solve_a(map: &Vec<Vec<char>>) -> usize {
    let w = map[0].len();
    let mut count = 0;
    for i in 0..map.len() {
        let x = (3 * i) % w;
        let y = i;
        if map[y][x] == '#' {
            count += 1;
        }
    }
    count
}

fn solve_b(map: &Vec<Vec<char>>) -> usize {
    let w = map[0].len();
    let mut product = 1;
    for (dx, dy) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut count = 0;
        for i in 0..(map.len() / dy) {
            let x = (dx * i) % w;
            let y = dy * i;
            if map[y][x] == '#' {
                count += 1;
            }
        }
        product *= count;
    }
    product
}

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    (solve_a(&map).to_string(), solve_b(&map).to_string())
}
