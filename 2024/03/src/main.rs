use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let input_iter = input.split("mul(").skip(1);
    for slice in input_iter {
        let mut s = slice.split(',');
        let first_part = s.next().unwrap();
        let Ok(num1) = first_part.parse::<u32>() else {
            //println!("slice: {}", slice);
            continue;
        };

        let Some(second_part) = s.next() else {
            //println!("slice: {}", slice);
            continue;
        };
        let Ok(num2) = second_part.split(')').next().unwrap().parse::<u32>()
        else {
            //println!("slice: {}", slice);
            continue;
        };
        sum += num1 * num2;
    }
    sum
}

fn part2(input: &str) -> u32 {
    // Stitching a do() at the beginning
    let input = &format!("do(){}", input);
    let find_active = Regex::new(r"do\(\)([\s\S]+?)don't\(\)").unwrap();
    let find_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for (_, [active_segment]) in
        find_active.captures_iter(input).map(|c| c.extract())
    {
        for (_, [num1, num2]) in
            find_mul.captures_iter(active_segment).map(|c| c.extract())
        {
            let num1: u32 = num1.parse().unwrap();
            let num2: u32 = num2.parse().unwrap();
            sum += num1 * num2;
        }
    }
    sum
}
