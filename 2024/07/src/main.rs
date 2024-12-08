fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut left_right = line.split(':');
        let result: i64 = left_right.next().unwrap().parse().unwrap();
        let mut values: Vec<i64> = left_right
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        values.reverse();

        if recursive_valid_check(result, &values) {
            sum += result;
        }
    }
    sum
}

// 2303: 64 1 2 18
// 3267: 81 40 27
fn recursive_valid_check(result: i64, values_rev: &[i64]) -> bool {
    let value = values_rev[0];

    if values_rev.len() == 1 {
        return result == value;
    }

    // Check if it is a '*' multiplication
    if result % value == 0 {
        let res = result / value;
        if recursive_valid_check(res, &values_rev[1..]) {
            return true;
        }
    }
    // If not, then it is addition
    let res = result - value;
    if recursive_valid_check(res, &values_rev[1..]) {
        return true;
    }

    false
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut left_right = line.split(':');
        let result: i64 = left_right.next().unwrap().parse().unwrap();
        let mut values: Vec<i64> = left_right
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        values.reverse();

        if recursive_valid_check_part2(result, &values) {
            sum += result;
        }
    }
    sum
}

// 7290: 6 8 6 15
fn recursive_valid_check_part2(result: i64, values_rev: &[i64]) -> bool {
    let value = values_rev[0];

    if values_rev.len() == 1 {
        return result == value;
    }

    if result < 0 {
        return false;
    }

    // Check if it is a '*' multiplication
    if result % value == 0 {
        let res = result / value;
        if recursive_valid_check_part2(res, &values_rev[1..]) {
            return true;
        }
    }

    // If not, check if it is concatenation
    let digit_count = value.ilog10() as i64 + 1;
    let mask = 10_i64.pow(digit_count as u32);
    let trailing_digits = result % mask;
    if trailing_digits == value {
        let res = result / mask;
        if recursive_valid_check_part2(res, &values_rev[1..]) {
            return true;
        }
    }

    // If not, then it is addition
    let res = result - value;
    if recursive_valid_check_part2(res, &values_rev[1..]) {
        return true;
    }

    false
}

#[test]
fn test_2024_07() {
    let result = 9870i64;
    let value = 70i64;
    let digit_count = value.ilog10() as i64 + 1;
    let mask = 10_i64.pow(digit_count as u32);
    let trailing_digits = result % mask;
    if trailing_digits == value {
        let res = result / mask;
        println!("YES, res: {}", res);
    }
    println!("END: count: {}, trailing: {}", digit_count, trailing_digits);
}
