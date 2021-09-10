use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve_a(passports: &[HashMap<&str, &str>]) -> usize {
    let keyset: HashSet<&&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .iter()
        .collect();

    passports
        .iter()
        .filter(|passport| {
            let mut passport_keys: HashSet<&&str> = passport.keys().collect();
            passport_keys.insert(&"cid");
            passport_keys == keyset
        })
        .count()
}

fn solve_b(passports: &[HashMap<&str, &str>]) -> usize {
    let keyset: HashSet<&&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .iter()
        .collect();
    let ecls: HashSet<&&str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .collect();
    passports
        .iter()
        .filter(|passport| {
            let mut passport_keys: HashSet<&&str> = passport.keys().collect();
            passport_keys.insert(&"cid");
            passport_keys == keyset
                && passport.iter().all(|(k, v)| match *k {
                    "byr" => {
                        v.len() == 4
                            && v.parse::<u16>()
                                .map(|byr| (1920..=2002).contains(&byr))
                                .unwrap_or(false)
                    }
                    "iyr" => {
                        v.len() == 4
                            && v.parse::<u16>()
                                .map(|iyr| (2010..=2020).contains(&iyr))
                                .unwrap_or(false)
                    }
                    "eyr" => {
                        v.len() == 4
                            && v.parse::<u16>()
                                .map(|eyr| (2020..=2030).contains(&eyr))
                                .unwrap_or(false)
                    }
                    "hgt" => {
                        let unit = &v[v.len() - 2..];
                        v[..v.len() - 2]
                            .parse::<u16>()
                            .map(|len| match unit {
                                "cm" => (150..=193).contains(&len),
                                "in" => (59..=76).contains(&len),
                                _ => false,
                            })
                            .unwrap_or(false)
                    }
                    "hcl" => {
                        v.len() == 7
                            && v.starts_with('#')
                            && v.chars()
                                .skip(1)
                                .all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
                    }
                    "ecl" => ecls.contains(v),
                    "pid" => v.len() == 9 && v.chars().all(|c| ('0'..='9').contains(&c)),
                    "cid" => true,
                    _ => unreachable!(),
                })
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let passports: Vec<HashMap<&str, &str>> =
        lines.iter().fold(vec![HashMap::new()], |mut result, line| {
            if line.is_empty() {
                result.push(HashMap::new());
            } else {
                for entry in line.split_whitespace() {
                    result.last_mut().unwrap().insert(&entry[0..3], &entry[4..]);
                }
            }
            result
        });

    (
        solve_a(&passports).to_string(),
        solve_b(&passports).to_string(),
    )
}
