fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../inputs/02.txt");
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum = 0;
    'games: for (i, line) in input.lines().enumerate() {
        let game_id = i + 1;
        let sets: Vec<_> =
            line.split(':').nth(1).unwrap().trim().split("; ").collect();
        for set in sets {
            let cubes_colors: Vec<_> = set.split(", ").collect();
            for cubes in cubes_colors {
                let split: Vec<_> = cubes.split(' ').collect();
                let number: u32 = split[0].parse().unwrap();
                let color = split[1];
                match color {
                    "red" => {
                        if number > max_red {
                            continue 'games;
                        }
                    }
                    "green" => {
                        if number > max_green {
                            continue 'games;
                        }
                    }
                    "blue" => {
                        if number > max_blue {
                            continue 'games;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        sum += game_id;
    }
    println!("sum: {sum}");
}

fn part2() {
    let input = include_str!("../inputs/02.txt");

    let mut sum = 0;
    for line in input.lines() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let sets: Vec<_> =
            line.split(':').nth(1).unwrap().trim().split("; ").collect();

        for set in sets {
            let cubes_colors: Vec<_> = set.split(", ").collect();

            for cubes in cubes_colors {
                let split: Vec<_> = cubes.split(' ').collect();
                let number: u32 = split[0].parse().unwrap();
                let color = split[1];
                match color {
                    "red" => {
                        max_red = max_red.max(number);
                    }
                    "green" => {
                        max_green = max_green.max(number);
                    }
                    "blue" => {
                        max_blue = max_blue.max(number);
                    }
                    _ => unreachable!(),
                }
            }
        }
        let power = max_red * max_blue * max_green;
        sum += power;
    }
    println!("sum: {sum}");
}
