use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

const keys: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

fn solve_a(passports: &[HashMap<&str, &str>], keyset: &HashSet<&&str>) -> usize {
    passports
        .iter()
        .filter(|passport| {
            let mut passport_keys: HashSet<&&str> = passport.keys().collect();
            passport_keys.insert(&&"cid");
            passport_keys == *keyset
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut passiter = lines.iter();
    let keyset = keys.iter().collect();

    let mut passports: Vec<HashMap<&str, &str>> =
        lines.iter().fold(vec![HashMap::new()], |mut result, line| {
            if line == "" {
                result.push(HashMap::new());
            } else {
                for entry in line.split_whitespace() {
                    result.last_mut().unwrap().insert(&entry[0..3], &entry[4..]);
                }
            }
            result
        });

    println!("{:?}", passports);
    (solve_a(&passports, &keyset).to_string(), "".to_string())
}
