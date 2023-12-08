use aoc_2023_03::part1;
use aoc_2023_03::part2;

fn main() {
    let input = include_str!("../input.txt");
    match std::env::args().nth(1).as_deref() {
        Some("all") => {
            let answer = part1(input);
            println!("Part 1 answer: {answer}");
            let answer = part2(input);
            println!("Part 2 answer: {answer}");
        }
        Some("one") | Some("1") => {
            let answer = part1(input);
            println!("Part 1 answer: {answer}");
        }
        Some("two") | Some("2") => {
            let answer = part2(input);
            println!("Part 2 answer: {answer}");
        }
        _ => println!(
            "Invalid argument!\nAvailable arguments: 'all', 'one', 'two'"
        ),
    }
}
