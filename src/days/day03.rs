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

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    (solve_a(&map).to_string(), "".to_string())
}
