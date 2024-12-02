fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result:{}", part1(input));
    println!("Part 2 result:{}", part2(input));
}

fn part1(input: &str) -> i32 {
    let elements_iter = input.split_whitespace();
    let mut left_row: Vec<i32> = elements_iter
        .clone()
        .step_by(2)
        .map(|elem| elem.parse::<i32>().unwrap())
        .collect();
    let mut right_row: Vec<i32> = elements_iter
        .skip(1)
        .step_by(2)
        .map(|elem| elem.parse::<i32>().unwrap())
        .collect();
    left_row.sort();
    right_row.sort();

    left_row
        .iter()
        .zip(right_row.iter())
        .map(|(left, right)| (left - right).abs())
        .sum()
}

fn part2(input: &str) -> u32 {
    let elements_iter: std::str::SplitWhitespace<'_> = input.split_whitespace();
    let left_row: Vec<u32> = elements_iter
        .clone()
        .step_by(2)
        .map(|elem| elem.parse::<u32>().unwrap())
        .collect();
    let right_row: Vec<u32> = elements_iter
        .skip(1)
        .step_by(2)
        .map(|elem| elem.parse::<u32>().unwrap())
        .collect();

    let mut sum = 0;
    for left in left_row {
        let occurences =
            right_row.iter().filter(|&&right| left == right).count() as u32;
        sum += occurences * left;
    }

    sum
}
