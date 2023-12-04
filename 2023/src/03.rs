use std::ops::RangeInclusive;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../inputs/03.txt");

    let lines: Vec<&str> = input.lines().collect();
    let line_len = input.lines().next().unwrap().len();

    let mut sum = 0;
    for (i, line) in input.lines().enumerate() {
        let mut position = 0;

        while let Some(rel_start) =
            line[position..].find(|c: char| c.is_ascii_digit())
        {
            let start = position + rel_start;
            let rel_end_bound = line[start..]
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(line_len - start);
            let end_pos = start + rel_end_bound - 1;
            position = end_pos + 1;
            let num: u32 = line[start..=end_pos].parse().unwrap();

            let bound_box_left = start.saturating_sub(1);
            let bound_box_right = (end_pos + 1).min(line_len - 1);

            if i != 0
                && lines[i - 1][bound_box_left..=bound_box_right]
                    .chars()
                    .any(|c| c != '.' && !c.is_ascii_digit())
            {
                sum += num;
                continue;
            }
            let c = lines[i].chars().nth(bound_box_left).unwrap();
            if c != '.' && !c.is_ascii_digit() {
                sum += num;
                continue;
            }
            let c = lines[i].chars().nth(bound_box_right).unwrap();
            if c != '.' && !c.is_ascii_digit() {
                sum += num;
                continue;
            }
            if i != lines.len() - 1
                && lines[i + 1][bound_box_left..=bound_box_right]
                    .chars()
                    .any(|c| c != '.' && !c.is_ascii_digit())
            {
                sum += num;
                continue;
            }
        }
    }
    println!("sum: {}", sum);
}

fn part2() {
    let input = include_str!("../inputs/03.txt");

    let lines: Vec<&str> = input.lines().collect();
    let line_len = input.lines().next().unwrap().len();

    let mut part_numbers = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut position = 0;
        let line_index = line_len * i;

        while let Some(rel_start) =
            line[position..].find(|c: char| c.is_ascii_digit())
        {
            let start = position + rel_start;
            let rel_end_bound = line[start..]
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(line_len - start);
            let end_pos = start + rel_end_bound - 1;
            position = end_pos + 1;
            let num: u32 = line[start..=end_pos].parse().unwrap();

            let abs_start = line_index + start;
            let abs_end = line_index + end_pos;

            let part_number = PartNumber {
                value: num,
                indices: abs_start..=abs_end,
            };
            part_numbers.push(part_number);
        }
    }

    let mut sum = 0;
    'outer: for (i, line) in input.lines().enumerate() {
        let line_index = line_len * i;

        for (index, _) in line.match_indices('*') {
            let mut bb_indices = Vec::new();
            let bb_left = if index == 0 {
                line_index
            } else {
                bb_indices.push(line_index + index - 1);
                line_index + index - 1
            };
            let bb_right = if index == line_len - 1 {
                line_index + index
            } else {
                bb_indices.push(line_index + index + 1);
                line_index + index + 1
            };
            if i != 0 {
                let abs_bound_box_top_left = bb_left - line_len;
                let abs_bound_box_top_right = bb_right - line_len;
                for i in abs_bound_box_top_left..=abs_bound_box_top_right {
                    bb_indices.push(i);
                }
            }

            if i != (lines.len() - 1) {
                let abs_bound_box_bottom_left = bb_left + line_len;
                let abs_bound_box_bottom_right = bb_right + line_len;
                for i in abs_bound_box_bottom_left..=abs_bound_box_bottom_right
                {
                    bb_indices.push(i);
                }
            }

            let mut touching = 0;
            let mut product = 1;
            for part_num in part_numbers.iter() {
                for i in part_num.indices.clone() {
                    if bb_indices.contains(&i) {
                        touching += 1;
                        if touching > 2 {
                            continue 'outer;
                        }
                        product *= part_num.value;
                        break;
                    }
                }
            }
            if touching == 2 {
                sum += product;
            }
        }
    }

    println!("sum: {}", sum);
}

struct PartNumber {
    value: u32,
    indices: RangeInclusive<usize>,
}
