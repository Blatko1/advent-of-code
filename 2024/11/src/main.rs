use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for stone in input.split_whitespace().map(|s| s.parse::<u64>().unwrap()) {
        sum += recursive_blink(stone, 0);
    }

    sum
}

fn recursive_blink(stone: u64, blink_count: u64) -> u64 {
    if blink_count == 25 {
        //print!("{} ", stone);
        return 1;
    }
    // If is equals to 0, change to 1
    if stone == 0 {
        return recursive_blink(1, blink_count+1);
    }

    // If the number of digits is even, split
    let num_len = stone.ilog10() + 1;
    if num_len % 2 == 0 {
        let half_len = num_len / 2;
        let mask = 10u64.pow(half_len);
        let first_half = stone / mask;
        let second_half = stone % mask;
        let recursive_result =
        recursive_blink(first_half, blink_count+1) +
        recursive_blink(second_half, blink_count+1);
        return recursive_result;
    }
    
    // Else, multiply by 2024
    return recursive_blink(stone * 2024, blink_count+1)
}

fn part2(input: &str) -> u64 {
    let mut memoize_map = HashMap::new();
    let mut sum = 0;
    for stone in input.split_whitespace().map(|s| s.parse::<u64>().unwrap()) {
        sum += recursive_blink_part2(stone, 0, &mut memoize_map);
    }

    sum
}

fn recursive_blink_part2(stone: u64, blink_count: u64, memoization: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blink_count == 75 {
        return 1;
    }
    if let Some(&result) = memoization.get(&(stone, blink_count)) {
        return result
    }

    // If is equals to 0, change to 1
    if stone == 0 {
        let result = recursive_blink_part2(1, blink_count+1, memoization);
        memoization.insert((stone, blink_count), result);
        return result
    }

    // If the number of digits is even, split
    let num_len = stone.ilog10() + 1;
    if num_len % 2 == 0 {
        let half_len = num_len / 2;
        let mask = 10u64.pow(half_len);
        let first_half = stone / mask;
        let second_half = stone % mask;
        let result =
        recursive_blink_part2(first_half, blink_count+1, memoization) +
        recursive_blink_part2(second_half, blink_count+1, memoization);
        memoization.insert((stone, blink_count), result);
        return result;
    }
    
    // Else, multiply by 2024
    let result = recursive_blink_part2(stone * 2024, blink_count+1,memoization);
    memoization.insert((stone, blink_count), result);
    result
}
