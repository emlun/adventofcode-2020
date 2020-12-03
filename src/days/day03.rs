use crate::common::Solution;

fn count(map: &[String], dx: usize, dy: usize) -> usize {
    let w = map[0].len();
    let mut count = 0;
    for i in 0..(map.len() / dy) {
        let x = (dx * i) % w;
        let y = dy * i;
        if map[y].chars().nth(x) == Some('#') {
            count += 1;
        }
    }
    count
}

fn solve_a(map: &[String]) -> usize {
    count(map, 3, 1)
}

fn solve_b(map: &[String], a_solution: usize) -> usize {
    let p: usize = vec![(1, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(dx, dy)| count(map, *dx, *dy))
        .product();
    p * a_solution
}

pub fn solve(lines: &[String]) -> Solution {
    let a_solution = solve_a(lines);
    (
        a_solution.to_string(),
        solve_b(lines, a_solution).to_string(),
    )
}
