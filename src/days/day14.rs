use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

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
        // println!("mask     {:036b}", mask);
        // println!("maskmask {:036b}", maskmask);
        } else if inst.starts_with("mem") {
            let addr = inst[4..(inst.find(']').unwrap())].parse().unwrap();
            if mem.len() <= addr {
                mem.resize(addr + 1, 0);
            }
            let value: u64 = value.parse().unwrap();
            mem[addr] = (value & maskmask) | mask;
            // println!("value    {:036b}", value);
            // println!("write    {:036b}", mem[addr]);
        }
        // println!();
    }
    mem.iter().sum()
}

fn solve_b(lines: &[String]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut maskmask: u64 = 0;
    let mut floating_masks: HashSet<u64> = HashSet::new();

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
            for (i, c) in value.chars().enumerate() {
                if c == 'X' {
                    let j = 36 - i - 1;
                    // dbg!(c, i, j);
                    let new_masks: Vec<u64> = floating_masks
                        .iter()
                        .map(|msk| msk | (1 << j))
                        .chain(floating_masks.iter().map(|msk| msk & !(1 << j)))
                        .collect();
                    floating_masks.extend(new_masks);
                }
            }

        // println!("pattern  {}", value);
        // println!("maskmask {:036b}", maskmask);
        // for msk in &floating_masks {
        // println!("mask     {:036b}", msk);
        // }
        } else if inst.starts_with("mem") {
            let addr_base: u64 = inst[4..(inst.find(']').unwrap())].parse().unwrap();
            let value: u64 = value.parse().unwrap();
            // println!("value    {:036b}", value);
            // println!("maskmask {:036b}", maskmask);
            // println!("addr     {:036b}", addr_base);

            for mask in &floating_masks {
                let addr = (addr_base & maskmask) | mask;
                mem.insert(addr, value);
                // println!("write at {:036b}", addr);
            }
        }
        // println!();
    }
    mem.values().sum()
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b(lines).to_string())
}
