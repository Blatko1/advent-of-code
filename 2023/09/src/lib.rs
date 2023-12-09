pub fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut sequences = Vec::new();
        let original_seq: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        sequences.push(original_seq);

        loop {
            let last_seq = sequences.last().unwrap();
            let len = last_seq.len();
            let differene_sequence: Vec<i64> = last_seq[0..(len - 1)]
                .iter()
                .zip(last_seq[1..].iter())
                .map(|(&a, &b)| b - a)
                .collect();
            let last_differnce = *differene_sequence.last().unwrap();
            sequences.push(differene_sequence);
            if last_differnce == 0 {
                break;
            }
        }

        let mut prediction = 0;
        for seq in sequences.iter().rev().skip(1) {
            prediction += seq.last().unwrap();
        }
        sum += prediction;
    }
    sum
}

pub fn part2(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut sequences = Vec::new();
        let original_seq: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        sequences.push(original_seq);

        loop {
            let last_seq = sequences.last().unwrap();
            let len = last_seq.len();
            let differene_sequence: Vec<i64> = last_seq[0..(len - 1)]
                .iter()
                .zip(last_seq[1..].iter())
                .map(|(&a, &b)| b - a)
                .collect();
            let last_differnce = *differene_sequence.last().unwrap();
            sequences.push(differene_sequence);
            if last_differnce == 0 {
                break;
            }
        }

        let mut prediction = 0;
        for seq in sequences.iter().rev().skip(1) {
            prediction = seq.first().unwrap() - prediction;
        }
        sum += prediction;
    }
    sum
}
