use crate::common::Solution;
use std::collections::HashMap;

pub fn solve(lines: &[String]) -> Solution {
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
            println!("mask     {:036b}", mask);
            println!("maskmask {:036b}", maskmask);
        } else if inst.starts_with("mem") {
            let addr = inst[4..(inst.find(']').unwrap())].parse().unwrap();
            if mem.len() <= addr {
                mem.resize(addr + 1, 0);
            }
            let value: u64 = value.parse().unwrap();
            mem[addr] = (value & maskmask) | mask;
            println!("value    {:036b}", value);
            println!("write    {:036b}", mem[addr]);
        }
        println!();
    }
    let a_solution: u64 = mem.iter().sum();

    (a_solution.to_string(), "".to_string())
}
