fn main() {
    let input = include_str!("../inputs/input01.txt");

    let mut sum = 0;
    for line in input.lines() {
        let mut number_words = Vec::with_capacity(line.len());
        for (num, num_str) in NUMBERS.iter().enumerate() {
            let matches: Vec<(usize, &str)> = line.match_indices(num_str).collect();
            for (index, _) in matches {
                number_words.push((num, index))
            }
        }
        number_words.sort_by(|(_, pos_a), (_, pos_b)| pos_a.cmp(pos_b));
        
        if !number_words.is_empty() {
            let (first_number_word, first_number_word_pos) =
                number_words.first().unwrap();
            let (last_number_word, last_number_word_pos) =
                number_words.last().unwrap();

            let (first_number_digit_pos, first_number_digit) = line
                .chars()
                .enumerate()
                .find(|(_, c)| c.is_ascii_digit())
                .unwrap();

            let (last_number_digit_pos_rev, last_number_digit) = line
                .chars()
                .rev()
                .enumerate()
                .find(|(_, c)| c.is_ascii_digit())
                .unwrap();
            let last_number_digit_pos =
                line.len() - 1 - last_number_digit_pos_rev;
                
            let first = if *first_number_word_pos < first_number_digit_pos {
                *first_number_word as u32
            } else {
                first_number_digit.to_digit(10).unwrap()
            };

            let last = if *last_number_word_pos > last_number_digit_pos {
                *last_number_word as u32
            } else {
                last_number_digit.to_digit(10).unwrap()
            };
            let num: i32 = format!("{first}{last}").parse().unwrap();
            sum += num;
        } else {
            let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last = match line.chars().rev().find(|c| c.is_ascii_digit()) {
                Some(c) => c,
                None => first,
            };
            let num: i32 = format!("{first}{last}").parse().unwrap();
            sum += num;
        }
    }
    println!("sum: {}", sum);
}

const NUMBERS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight",
    "nine",
];
