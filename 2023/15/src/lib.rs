pub fn part1(input: &str) -> u64 {
    let steps: Vec<&str> = input.split(',').collect();

    let mut sum = 0;
    for step in steps {
        // Hashing algorithm
        let mut value = 0;
        for c in step.chars() {
            value += c as u64;
            value *= 17;
            value %= 256;
        }
        sum += value;
    }

    sum
}

pub fn part2(_input: &str) -> u64 {
    todo!();
}
