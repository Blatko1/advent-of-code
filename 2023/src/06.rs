fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../inputs/06.txt");
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap());

    let mut product = 1;
    times.zip(distances).for_each(|(race_time, distance)| {
        let time = race_time as f32;
        let dist = distance as f32;

        // Calculate the first winning time with quadratic formula
        let mut first_win_time =
            ((time - (time * time - 4.0 * dist).sqrt()) / 2.0).ceil() as u64;

        if first_win_time * (race_time - first_win_time) == distance {
            first_win_time += 1;
        }

        let last_win_time = race_time - first_win_time;

        let win_count = last_win_time - first_win_time + 1;
        product *= win_count;
    });
    println!("product: {}", product);
}

fn part2() {
    let input = include_str!("../inputs/06.txt");
    let mut lines = input.lines();

    let race_time = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .to_owned()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .to_owned()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let time = race_time as f32;
    let dist = distance as f32;

    // Calculate the first winning time with quadratic formula
    let mut first_win_time =
        ((time - (time * time - 4.0 * dist).sqrt()) / 2.0).ceil() as u64;

    if first_win_time * (race_time - first_win_time) == distance {
        first_win_time += 1;
    }

    let last_win_time = race_time - first_win_time;

    let win_count = last_win_time - first_win_time + 1;

    println!("win_count: {}", win_count);
}
