mod hands;
use crate::hands::hand1::Hand as Hand1;
pub fn part1(input: &str) -> u64 {
    let lines = input.lines().map(|l| {
        let input: Vec<&str> = l.split_whitespace().collect();
        (input[0], input[1].parse::<u64>().unwrap())
    });

    let mut hands = Vec::new();
    lines.for_each(|(hand_str, bid)| {
        let hand = Hand1::from(hand_str);
        hands.push((hand, bid))
    });
    hands.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let mut sum = 0;
    for (f, &(_, bid)) in hands.iter().enumerate() {
        let factor = f as u64 + 1;
        sum += bid * factor;
    }
    sum
}

use crate::hands::hand2::Hand as Hand2;
pub fn part2(input: &str) -> u64 {
    let lines = input.lines().map(|l| {
        let input: Vec<&str> = l.split_whitespace().collect();
        (input[0], input[1].parse::<u64>().unwrap())
    });

    let mut hands = Vec::new();
    lines.for_each(|(hand_str, bid)| {
        let hand = Hand2::from(hand_str);
        hands.push((hand, bid))
    });
    hands.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let mut sum = 0;
    for (f, &(_, bid)) in hands.iter().enumerate() {
        let factor = f as u64 + 1;
        sum += bid * factor;
    }
    sum
}
