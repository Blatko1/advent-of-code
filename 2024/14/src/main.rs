use std::cmp::Ordering;

use regex::Regex;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut top_left_quadrant = 0;
    let mut top_right_quadrant = 0;
    let mut bottom_left_quadrant = 0;
    let mut bottom_right_quadrant = 0;
    for (_, [x, y, vx, vy]) in regex.captures_iter(input).map(|c| c.extract()) {
        let pos_x: i32 = x.parse().unwrap();
        let pos_y: i32 = y.parse().unwrap();
        let vel_x: i32 = vx.parse().unwrap();
        let vel_y: i32 = vy.parse().unwrap();

        let time = 100;
        let new_pos_x = (pos_x + vel_x * time).rem_euclid(WIDTH);
        let new_pos_y = (pos_y + vel_y * time).rem_euclid(HEIGHT);
        match new_pos_x.cmp(&(WIDTH / 2)) {
            Ordering::Less => match new_pos_y.cmp(&(HEIGHT / 2)) {
                Ordering::Less => {
                    top_left_quadrant += 1;
                }
                Ordering::Greater => {
                    bottom_left_quadrant += 1;
                }
                _ => (),
            },
            Ordering::Greater => match new_pos_y.cmp(&(HEIGHT / 2)) {
                Ordering::Less => {
                    top_right_quadrant += 1;
                }
                Ordering::Greater => {
                    bottom_right_quadrant += 1;
                }
                _ => (),
            },
            _ => (),
        }
    }
    top_left_quadrant
        * top_right_quadrant
        * bottom_left_quadrant
        * bottom_right_quadrant
}

fn part2(input: &str) -> u32 {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<(i32, i32, i32, i32)> = regex
        .captures_iter(input)
        .map(|c| {
            let data: [&str; 4] = c.extract().1;
            (
                data[0].parse().unwrap(),
                data[1].parse().unwrap(),
                data[2].parse().unwrap(),
                data[3].parse().unwrap(),
            )
        })
        .collect();
    let mut empty_row: String = (0..WIDTH).map(|_| '.').collect();
    empty_row.push('\n');
    let max = 10000000;
    'outer: for second in 1..=max {
        let mut map: String =
            (0..HEIGHT).flat_map(|_| empty_row.chars()).collect();
        for (x, y, vel_x, vel_y) in robots.iter_mut() {
            *x = (*x + *vel_x).rem_euclid(WIDTH);
            *y = (*y + *vel_y).rem_euclid(HEIGHT);
            let idx = ((*y) * empty_row.len() as i32 + (*x)) as usize;
            map.replace_range(idx..idx + 1, "X");
            //std::fs::write("2024/14/out.txt", map.clone()).unwrap();
        }
        if map.contains("XXXXXXX") {
            println!("found at {}", second);
            std::fs::write("2024/14/out.txt", map).unwrap();
            break 'outer;
        }
    }
    todo!()
}
