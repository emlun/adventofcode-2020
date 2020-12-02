use crate::common::Solution;

fn solve_a(entries: &[(usize, usize, char, &str)]) -> usize {
    entries
        .iter()
        .filter(|(min, max, passchar, password)| {
            let charcount = password.chars().filter(|c| c == passchar).count();
            charcount >= *min && charcount <= *max
        })
        .count()
}

fn solve_b(entries: &[(usize, usize, char, &str)]) -> usize {
    entries
        .iter()
        .filter(|(pos1, pos2, passchar, password)| {
            let a = password.chars().nth(pos1 - 1) == Some(*passchar);
            let b = password.chars().nth(pos2 - 1) == Some(*passchar);
            a && !b || !a && b
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let entries: Vec<(usize, usize, char, &str)> = lines
        .iter()
        .map(|line| {
            let mut parts1 = line.split(": ");
            let policy = parts1.next().unwrap();
            let password = parts1.next().unwrap();
            let mut parts2 = policy.split(' ');
            let minmax = parts2.next().unwrap();
            let passchar = parts2.next().unwrap();
            let mut parts3 = minmax.split('-');
            (
                parts3.next().unwrap().parse().unwrap(),
                parts3.next().unwrap().parse().unwrap(),
                passchar.chars().next().unwrap(),
                password,
            )
        })
        .collect();

    (solve_a(&entries).to_string(), solve_b(&entries).to_string())
}
