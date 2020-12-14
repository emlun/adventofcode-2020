use crate::common::Solution;
use std::collections::BTreeSet;
use std::collections::HashMap;

fn solve_a(lines: &[String]) -> u64 {
    let mut mask: u64 = 0;
    let mut maskmask: u64 = 0;
    let mut mem: Vec<u64> = Vec::new();

    for line in lines.iter().filter(|l| !l.is_empty()) {
        let mut parts = line.split('=').map(|s| s.trim());
        let inst = parts.next().unwrap();
        let value = parts.next().unwrap();
        if inst == "mask" {
            maskmask = u64::from_str_radix(&value.replace('1', "0").replace('X', "1"), 2).unwrap();
            mask = u64::from_str_radix(&value.replace('X', "0"), 2).unwrap();
        } else if inst.starts_with("mem") {
            let addr = inst[4..(inst.find(']').unwrap())].parse().unwrap();
            if mem.len() <= addr {
                mem.resize(addr + 1, 0);
            }
            let value: u64 = value.parse().unwrap();
            mem[addr] = (value & maskmask) | mask;
        }
    }
    mem.iter().sum()
}

fn solve_b(lines: &[String]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut maskmask: u64 = 0;
    let mut floating_masks: BTreeSet<u64> = BTreeSet::new();

    for line in lines.iter().filter(|l| !l.is_empty()) {
        let mut parts = line.split('=').map(|s| s.trim());
        let inst = parts.next().unwrap();
        let value = parts.next().unwrap();

        if inst == "mask" {
            floating_masks.clear();
            let mask_base = u64::from_str_radix(&value.replace('X', "0"), 2).unwrap();
            maskmask = (!u64::from_str_radix(&value.replace('1', "0").replace('X', "1"), 2)
                .unwrap())
                % (1 << 36);
            floating_masks.insert(mask_base);
            for (i, _) in value.chars().enumerate().filter(|(_, c)| c == &'X') {
                let new_maskmask = 1 << (36 - i - 1);
                floating_masks.extend(
                    floating_masks
                        .iter()
                        .map(|msk| msk | new_maskmask)
                        .chain(floating_masks.iter().map(|msk| msk & !new_maskmask))
                        .collect::<Vec<u64>>(),
                );
            }
        } else if inst.starts_with("mem") {
            let addr_base: u64 = inst[4..(inst.find(']').unwrap())].parse().unwrap();
            let value: u64 = value.parse().unwrap();
            for mask in &floating_masks {
                let addr = (addr_base & maskmask) | mask;
                mem.insert(addr, value);
            }
        }
    }
    mem.values().sum()
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b(lines).to_string())
}
