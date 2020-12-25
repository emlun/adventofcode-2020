use crate::common::Solution;

fn transform(mut val: usize, sub: usize, times: usize) -> usize {
    for _ in 0..times {
        val = (sub * val) % 20201227;
    }
    val
}

pub fn solve(lines: &[String]) -> Solution {
    let pub1 = lines[0].parse().unwrap();
    let pub2 = lines[1].parse().unwrap();
    const SUB: usize = 7;
    let mut val = 1;

    let mut pri = 0;
    while val != pub1 && val != pub2 {
        val = transform(val, SUB, 1);
        pri += 1;
    }

    let key = transform(1, if val == pub1 { pub2 } else { pub1 }, pri);

    (key.to_string(), "".to_string())
}
