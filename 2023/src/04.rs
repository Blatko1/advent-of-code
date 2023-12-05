fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../inputs/04.txt");

    let mut sum = 0;
    for line in input.lines() {
        let content = line.split(':').last().unwrap().trim();
        let split: Vec<_> =
            content.split('|').map(|nums| nums.trim()).collect();
        let winning_numbers: Vec<u32> = split[0]
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let mut available_numbers: Vec<u32> = split[1]
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        available_numbers.sort();

        let mut win_count = 0;
        for num in winning_numbers {
            if available_numbers.binary_search(&num).is_ok() {
                win_count += 1;
            }
        }
        match win_count.cmp(&1) {
            std::cmp::Ordering::Equal => sum += 1,
            std::cmp::Ordering::Greater => sum += 2u32.pow(win_count - 1),
            _ => (),
        }
    }
    println!("sum: {}", sum);
}

fn part2() {
    let input = include_str!("../inputs/04.txt");
    let mut instance_state = vec![1; input.lines().count()];

    for (card_i, line) in input.lines().enumerate() {
        let content = line.split(':').last().unwrap().trim();
        let split: Vec<_> =
            content.split('|').map(|nums| nums.trim()).collect();
        let winning_numbers: Vec<u32> = split[0]
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let mut available_numbers: Vec<u32> = split[1]
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        available_numbers.sort();

        let mut win_count = 0;
        for num in winning_numbers {
            if available_numbers.binary_search(&num).is_ok() {
                win_count += 1;
            }
        }
        for i in (card_i + 1)..=(card_i + win_count) {
            instance_state[i] += instance_state[card_i];
        }
    }
    let sum: u32 = instance_state.iter().sum();
    println!("sum: {}", sum);
}
