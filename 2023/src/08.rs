use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../inputs/08.txt");

    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().map(Direction::from);
    lines.next();
    let mut hmap = HashMap::new();
    for map in lines {
        let mut split = map.split(" = ");
        let index = split.next().unwrap();
        let mut paths = split.next().unwrap()[1..=8].split(", ");
        let l_path = paths.next().unwrap();
        let r_path = paths.next().unwrap();

        hmap.insert(index, (l_path, r_path));
    }

    let mut steps = 0;
    let mut paths = hmap.get("AAA").unwrap();
    'outer: loop {
        for dir in directions.clone() {
            steps += 1;
            let next = match dir {
                Direction::Left => {
                    if paths.0 == "ZZZ" {
                        break 'outer;
                    }
                    hmap.get(paths.0).unwrap()
                }
                Direction::Right => {
                    if paths.1 == "ZZZ" {
                        break 'outer;
                    }
                    hmap.get(paths.1).unwrap()
                }
            };
            paths = next;
        }
    }
    println!("steps: {}", steps);
}

fn part2() {
    let input = include_str!("../inputs/08.txt");

    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().map(Direction::from);
    lines.next();
    let mut hmap = HashMap::new();
    for map in lines {
        let mut split = map.split(" = ");
        let index = split.next().unwrap();
        let mut paths = split.next().unwrap()[1..=8].split(", ");
        let l_path = paths.next().unwrap();
        let r_path = paths.next().unwrap();

        hmap.insert(index, (l_path, r_path));
    }

    let start_paths_ending_a = hmap
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| hmap.get(key).unwrap());
    let mut steps = vec![1; start_paths_ending_a.clone().count()];
    for (i, start_paths) in start_paths_ending_a.enumerate() {
        let mut paths = start_paths;
        'outer: loop {
            for dir in directions.clone() {
                let next = match dir {
                    Direction::Left => {
                        if paths.0.ends_with('Z') {
                            break 'outer;
                        }
                        hmap.get(paths.0).unwrap()
                    }
                    Direction::Right => {
                        if paths.1.ends_with('Z') {
                            break 'outer;
                        }
                        hmap.get(paths.1).unwrap()
                    }
                };
                steps[i] += 1;
                paths = next;
            }
        }
    }
    let mut least_common_multiple = 1;
    for step in steps {
        least_common_multiple = lcm(least_common_multiple, step);
    }

    println!("least common multiple: {:?}", least_common_multiple);
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}
