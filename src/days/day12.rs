use crate::common::Solution;

fn solve_a(instructions: &[(char, i32)]) -> i32 {
    let mut ship_dir = 0;
    let mut ship_x = 0;
    let mut ship_y = 0;
    for (inst_dir, inst_len) in instructions {
        match inst_dir {
            'N' => ship_y += inst_len,
            'S' => ship_y -= inst_len,
            'E' => ship_x += inst_len,
            'W' => ship_x -= inst_len,
            'L' => ship_dir = (ship_dir + inst_len + 360) % 360,
            'R' => ship_dir = (ship_dir - inst_len + 360) % 360,
            'F' => match ship_dir {
                0 => ship_x += inst_len,
                90 => ship_y += inst_len,
                180 => ship_x -= inst_len,
                270 => ship_y -= inst_len,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    ship_x.abs() + ship_y.abs()
}

fn solve_b(instructions: &[(char, i32)]) -> i32 {
    let mut ship_x = 0;
    let mut ship_y = 0;

    let mut wp_dx = 10;
    let mut wp_dy = 1;

    for (inst_dir, inst_len) in instructions {
        match inst_dir {
            'N' => wp_dy += inst_len,
            'S' => wp_dy -= inst_len,
            'E' => wp_dx += inst_len,
            'W' => wp_dx -= inst_len,
            'L' => {
                for _ in 0..(inst_len / 90) {
                    let tmp = (-wp_dy, wp_dx);
                    wp_dx = tmp.0;
                    wp_dy = tmp.1;
                }
            }
            'R' => {
                for _ in 0..(inst_len / 90) {
                    let tmp = (wp_dy, -wp_dx);
                    wp_dx = tmp.0;
                    wp_dy = tmp.1;
                }
            }
            'F' => {
                ship_x += wp_dx * inst_len;
                ship_y += wp_dy * inst_len;
            }
            _ => unreachable!(),
        }
    }
    ship_x.abs() + ship_y.abs()
}

pub fn solve(lines: &[String]) -> Solution {
    let instructions: Vec<(char, i32)> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            (
                l.chars().next().unwrap(),
                l.chars().skip(1).collect::<String>().parse().unwrap(),
            )
        })
        .collect();
    (
        solve_a(&instructions).to_string(),
        solve_b(&instructions).to_string(),
    )
}
