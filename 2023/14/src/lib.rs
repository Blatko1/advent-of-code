pub fn part1(input: &str) -> u64 {
    let row_count = input.lines().count();
    let line_len = input.lines().next().unwrap().chars().count();
    let columns: Vec<String> = (0..line_len)
        .map(|i| {
            let mut column = String::with_capacity(row_count);
            for j in 0..row_count {
                // Watch out on offsetting to skip the '\n' and '\r'
                column
                    .push(input.chars().nth(i + j * 2 + j * line_len).unwrap());
            }
            column
        })
        .collect();

    let mut sum = 0;
    for column in columns {
        let mut base = row_count;
        for (stone, i) in column.chars().zip((1..=row_count).rev()) {
            match stone {
                '#' => base = i.saturating_sub(1),
                'O' => {
                    sum += base;
                    base -= 1;
                }
                '.' => (),
                _ => unreachable!(),
            }
            //println!("stone: {}, i: {}", stone, i);
        }
    }

    sum as u64
}

pub fn part2(_input: &str) -> u64 {
    todo!();
}
