use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut map: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let mut antena_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, &c)| {
            if c != '.' {
                antena_map.entry(c).or_default().push((x as i32, y as i32));
            }
        })
    });

    for (_, positions) in antena_map.iter() {
        let count = positions.len();
        for (i, (x1, y1)) in positions[0..count-1].iter().enumerate() {
            for (x2, y2) in &positions[(i+1)..count] {
                let offset = (x2 - x1, y2 - y1);
                // First antinode
                // Not checking for negatives since casting wraps around bounds
                let pos_x = (x1 - offset.0) as usize;
                let pos_y = (y1 - offset.1) as usize;
                if let Some(y_row) = map.get_mut(pos_y) {
                    if let Some(field) = y_row.get_mut(pos_x) {
                        *field = '#';
                    }
                };

                // Second antinode
                let pos_x = (x2 + offset.0) as usize;
                let pos_y = (y2 + offset.1) as usize;
                if let Some(y_row) = map.get_mut(pos_y) {
                    if let Some(field) = y_row.get_mut(pos_x) {
                        *field = '#';
                    }
                };
            }
        }
    }
    let out: String = map.iter().map(|row| 
        {let mut row_str: String = row.iter().collect(); 
            row_str.push('\n'); 
            row_str
        }
    ).collect();
    std::fs::write("2024/08/test.txt", &out).unwrap();

    out.chars().filter(|&c| c == '#').count()
}

fn part2(input: &str) -> usize {
    let mut map: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let mut antena_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, &c)| {
            if c != '.' {
                antena_map.entry(c).or_default().push((x as i32, y as i32));
            }
        })
    });

    for (_, positions) in antena_map.iter() {
        let count = positions.len();
        for (i, (x1, y1)) in positions[0..count-1].iter().enumerate() {
            map[*y1 as usize][*x1 as usize] = '#';
            for (x2, y2) in &positions[(i+1)..count] {
                map[*y2 as usize][*x2 as usize] = '#';

                let offset = (x2 - x1, y2 - y1);

                // First direction
                // Loops while there is space to place antinodes
                let (mut x, mut y) = (*x1, *y1);
                loop {
                    // Not checking for negatives since casting wraps around bounds
                    x -= offset.0;
                    y -= offset.1;
                    if let Some(y_row) = map.get_mut(y as usize) {
                        if let Some(field) = y_row.get_mut(x as usize) {
                            *field = '#';
                            continue;
                        }
                    };
                    break;
                }

                // Second direction
                // Loops while there is space to place antinodes
                let (mut x, mut y) = (*x2, *y2);
                loop {
                    x += offset.0;
                    y += offset.1;
                    if let Some(y_row) = map.get_mut(y as usize) {
                        if let Some(field) = y_row.get_mut(x as usize) {
                            *field = '#';
                            continue;
                        }
                    };
                    break;
                }
            }
        }
    }
    let out: String = map.iter().map(|row| 
        {let mut row_str: String = row.iter().collect(); 
            row_str.push('\n'); 
            row_str
        }
    ).collect();
    std::fs::write("2024/08/test2.txt", &out).unwrap();

    out.chars().filter(|&c| c == '#').count()
}

