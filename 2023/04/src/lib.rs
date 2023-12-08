pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let content = line.split(':').last().unwrap().trim();
        let split: Vec<_> =
            content.split('|').map(|nums| nums.trim()).collect();
        let winning_numbers: Vec<u64> = split[0]
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let mut available_numbers: Vec<u64> = split[1]
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
            std::cmp::Ordering::Greater => sum += 2u64.pow(win_count - 1),
            _ => (),
        }
    }
    sum
}

pub fn part2(input: &str) -> u64 {
    let mut instance_state = vec![1; input.lines().count()];

    for (card_i, line) in input.lines().enumerate() {
        let content = line.split(':').last().unwrap().trim();
        let split: Vec<_> =
            content.split('|').map(|nums| nums.trim()).collect();
        let winning_numbers: Vec<u64> = split[0]
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let mut available_numbers: Vec<u64> = split[1]
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
    let sum: u64 = instance_state.iter().sum();
    sum
}
