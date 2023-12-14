pub fn part1(input: &str) -> u64 {
    let sections = parse_input(input);

    let mut sum = 0;
    for section in sections {
        if let Some(horizontal) = find_reflection_lines(&section) {
            sum += 100 * horizontal;
            continue;
        }

        let rotated_owned = rotate_sections(&section);
        let rotated: Vec<&str> = rotated_owned.iter().map(|str| str.as_str()).collect();
        // For the vertical reflected lines
        if let Some(vertical) = find_reflection_lines(&rotated) {
            sum += vertical;
            continue;
        }
        unreachable!()
    }
    sum as u64
}

pub fn part2(input: &str) -> u64 {
    todo!();
}

/// Returns number of lines from top to the line near the reflection axis. 
fn find_reflection_lines(section: &[&str]) -> Option<usize> {
    // First check the first line and find the first matching line from
    // the back where all the lines in the middle are proven mirrored
    let first_row = section.first().unwrap();
    'outer: for (i, row) in section[1..].iter().enumerate().rev() {
        let index = i + 1;
        let reflected_rows = index + 1;
        // Count of the reflected rows should be even
        if reflected_rows % 2 != 0 {
            continue;
        }
        // If the first row matches the current row,
        // check if all rows in between are mirrored
        if first_row == row {
            let half = reflected_rows / 2;
            if !section[0..half]
                .iter()
                .zip(section[half..reflected_rows].iter().rev())
                .all(|(a, b)| a == b)
            {
                continue 'outer;
            }
            return Some(reflected_rows / 2);
        }
    }

    // First check the last line and find the first matching line from
    // the back where all the lines in the middle are proven mirrored
    let last_row = section.last().unwrap();
    let rows = section.len();
    'outer: for (index, row) in section[0..(rows-1)].iter().enumerate() {
        let reflected_rows = rows - index;
        // Count of the reflected rows should be even
        if reflected_rows % 2 != 0 {
            continue;
        }
        // If the first row matches the current row,
        // check if all rows in between are mirrored
        if last_row == row {
            let half = index + reflected_rows / 2;
            if !section[index..half]
                .iter()
                .zip(section[half..].iter().rev())
                .all(|(a, b)| a == b)
            {
                continue 'outer;
            }
            return Some(index + reflected_rows / 2);
        }
    }
    None
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    let mut sections = Vec::new();
    let mut section = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            sections.push(section.clone());
            section.clear();
            continue;
        }
        section.push(line);
    }
    sections.push(section);
    sections
}

fn rotate_sections<'a>(sections: &[&str]) -> Vec<String> {
    let height = sections.len();
    let line_len = sections.first().unwrap().len();
    let mut rotated_sections: Vec<String> = vec![String::with_capacity(sections.len()); line_len];
    for i in 0..line_len {
        for j in 0..sections.len() {
            rotated_sections[i].insert(j, sections[height - j - 1].chars().nth(i).unwrap());
        }
    }
    
    rotated_sections
}

#[test]
#[rustfmt::skip]
fn test_vertical_line() {
    let section = 
        &["#...##..#",
          "#....#..#", 
          "..##..###",
          "#####.##.",
          "#####.##.",
          "..##..###",
          "#....#..#"];
    assert_eq!(find_reflection_lines(section), Some(5));
}
