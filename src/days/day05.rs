use crate::common::Solution;
use std::cmp::Ordering;

fn parse_binary(result: usize, c: char, zero_char: char) -> usize {
    if c == zero_char {
        result << 1
    } else {
        (result << 1) | 1
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
    let mut mini = 0;
    let mut maxi = seats.len();
    loop {
        if maxi <= mini {
            return seats[mini] + 1;
        } else {
            let mid = (mini + maxi) / 2;

            match seats[mid].cmp(&(mid + 48)) {
                Ordering::Greater => maxi = mid - 1,
                _ => mini = mid,
            };
        }
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let seats = parse_seat_ids(lines);
    (solve_a(&seats).to_string(), solve_b(&seats).to_string())
}
