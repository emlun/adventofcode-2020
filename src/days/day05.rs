use crate::common::Solution;

fn parse_binary(result: usize, c: char, zero_char: char) -> usize {
    if c == zero_char {
        result * 2
    } else {
        result * 2 + 1
    }
}

fn parse_seat_ids(passes: &[String]) -> Vec<usize> {
    let mut seats: Vec<usize> = passes
        .iter()
        .map(|s| {
            let row: usize = s[0..7]
                .chars()
                .fold(0, |result, c| parse_binary(result, c, 'F'));
            let col: usize = s[7..]
                .chars()
                .fold(0, |result, c| parse_binary(result, c, 'L'));
            row * 8 + col
        })
        .collect();
    seats.sort();
    seats
}

fn solve_a(seats: &[usize]) -> usize {
    *seats.last().unwrap()
}

fn solve_b(seats: &[usize]) -> usize {
    for i in 0..seats.len() {
        let myid = seats[i] + 1;
        if seats[i + 1] > myid {
            return myid;
        }
    }
    unreachable!()
}

pub fn solve(lines: &[String]) -> Solution {
    let seats = parse_seat_ids(lines);
    (solve_a(&seats).to_string(), solve_b(&seats).to_string())
}
