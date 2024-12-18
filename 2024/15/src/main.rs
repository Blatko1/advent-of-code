use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut map: Vec<Vec<char>> = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let mut start_pos = (usize::MAX, usize::MAX);
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == '@') {
            start_pos = (x, y);
            break;
        }
    }

    let instructions = input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .flat_map(|line| line.chars());

    for instr in instructions {
        start_pos = match instr {
            '^' => move_bot_up(start_pos.0, start_pos.1, &mut map),
            '>' => move_bot_right(start_pos.0, start_pos.1, &mut map),
            'v' => move_bot_down(start_pos.0, start_pos.1, &mut map),
            '<' => move_bot_left(start_pos.0, start_pos.1, &mut map),
            _ => unreachable!(),
        }
    }
    let total: usize = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'O')
                .map(|(x, _)| 100 * y + x)
                .sum::<usize>()
        })
        .sum();
    for row in map.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!()
    }

    total
}

fn move_bot_up(
    start_x: usize,
    start_y: usize,
    map: &mut [Vec<char>],
) -> (usize, usize) {
    let mut y = start_y;
    while y - 1 > 0 {
        y -= 1;
        match map[y][start_x] {
            '#' => break,
            'O' => continue,
            // If an empty space is reached, move the bot and everything on it's way upwards
            '.' => {
                map[start_y][start_x] = '.';
                // Check if we only have to move the bot up by one
                if y + 1 == start_y {
                    map[y][start_x] = '@';
                    return (start_x, y);
                } else {
                    map[y][start_x] = 'O';
                    map[start_y - 1][start_x] = '@';
                    return (start_x, start_y - 1);
                }
            }
            _ => unreachable!(),
        }
    }
    (start_x, start_y)
}

fn move_bot_down(
    start_x: usize,
    start_y: usize,
    map: &mut [Vec<char>],
) -> (usize, usize) {
    let bottom_row = map.len() - 1;
    let mut y = start_y;
    while y + 1 < bottom_row {
        y += 1;
        match map[y][start_x] {
            '#' => break,
            'O' => continue,
            // If an empty space is reached, move the bot and everything on it's way downwards
            '.' => {
                map[start_y][start_x] = '.';
                // Check if we only have to move the bot down by one
                if y - 1 == start_y {
                    map[y][start_x] = '@';
                    return (start_x, y);
                } else {
                    map[y][start_x] = 'O';
                    map[start_y + 1][start_x] = '@';
                    return (start_x, start_y + 1);
                }
            }
            _ => unreachable!(),
        }
    }
    (start_x, start_y)
}

fn move_bot_left(
    start_x: usize,
    start_y: usize,
    map: &mut [Vec<char>],
) -> (usize, usize) {
    let mut x = start_x;
    while x - 1 > 0 {
        x -= 1;
        match map[start_y][x] {
            '#' => break,
            'O' => continue,
            // If an empty space is reached, move the bot and everything on it's way to the left
            '.' => {
                map[start_y][start_x] = '.';
                // Check if we only have to move the bot left by one
                if x + 1 == start_x {
                    map[start_y][x] = '@';
                    return (x, start_y);
                } else {
                    map[start_y][x] = 'O';
                    map[start_y][start_x - 1] = '@';
                    return (start_x - 1, start_y);
                }
            }
            _ => unreachable!(),
        }
    }
    (start_x, start_y)
}

fn move_bot_right(
    start_x: usize,
    start_y: usize,
    map: &mut [Vec<char>],
) -> (usize, usize) {
    let most_right_column = map[0].len() - 1;
    let mut x = start_x;
    while x + 1 < most_right_column {
        x += 1;
        match map[start_y][x] {
            '#' => break,
            'O' => continue,
            // If an empty space is reached, move the bot and everything on it's way to the right
            '.' => {
                map[start_y][start_x] = '.';
                // Check if we only have to move the bot right by one
                if x + 1 == start_x {
                    map[start_y][x] = '@';
                    return (x, start_y);
                } else {
                    map[start_y][x] = 'O';
                    map[start_y][start_x + 1] = '@';
                    return (start_x + 1, start_y);
                }
            }
            _ => unreachable!(),
        }
    }
    (start_x, start_y)
}

fn part2(input: &str) -> usize {
    let mut map: Vec<Vec<char>> = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            line.replace("#", "##")
                .replace("O", "[]")
                .replace(".", "..")
                .replace("@", "@.")
                .chars()
                .collect()
        })
        .collect();
    let mut start_pos = (usize::MAX, usize::MAX);
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == '@') {
            start_pos = (x, y);
            break;
        }
    }

    let instructions = input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .flat_map(|line| line.chars());

    for instr in instructions {
        start_pos = match instr {
            '^' => {
                let old_map = map.clone();
                let mut start_pos = start_pos;
                if move_up_recursive(
                    start_pos.0,
                    start_pos.1,
                    &mut map,
                    &old_map,
                ) {
                    map[start_pos.1][start_pos.0] = '.';
                    start_pos.1 -= 1;
                    if map[start_pos.1 - 1][start_pos.0] == '[' {
                        if !(map[start_pos.1][start_pos.0 + 1] == '['
                            && map[start_pos.1][start_pos.0 + 2] == ']')
                        {
                            map[start_pos.1][start_pos.0 + 1] = '.';
                        }
                    } else if map[start_pos.1 - 1][start_pos.0] == ']' {
                        if !(map[start_pos.1][start_pos.0 - 2] == '['
                            && map[start_pos.1][start_pos.0 - 1] == ']')
                        {
                            map[start_pos.1][start_pos.0 - 1] = '.';
                        }
                    }
                } else {
                    map = old_map
                }
                start_pos
            }
            '>' => move_bot_right_part2(start_pos.0, start_pos.1, &mut map),
            'v' => {
                let old_map = map.clone();
                let mut start_pos = start_pos;
                if move_down_recursive(
                    start_pos.0,
                    start_pos.1,
                    &mut map,
                    &old_map,
                ) {
                    map[start_pos.1][start_pos.0] = '.';
                    start_pos.1 += 1;
                    if map[start_pos.1 + 1][start_pos.0] == '[' {
                        if !(map[start_pos.1][start_pos.0 + 1] == '['
                            && map[start_pos.1][start_pos.0 + 2] == ']')
                        {
                            map[start_pos.1][start_pos.0 + 1] = '.';
                        }
                    } else if map[start_pos.1 + 1][start_pos.0] == ']' {
                        if !(map[start_pos.1][start_pos.0 - 2] == '['
                            && map[start_pos.1][start_pos.0 - 1] == ']')
                        {
                            map[start_pos.1][start_pos.0 - 1] = '.';
                        }
                    }
                } else {
                    map = old_map
                }
                start_pos
            }
            '<' => move_bot_left_part2(start_pos.0, start_pos.1, &mut map),
            _ => unreachable!(),
        };
        println!("\ninstr: {}", instr);
        for row in map.iter() {
            for c in row.iter() {
                print!("{}", c);
            }
            println!()
        }
        std::io::stdin().read_line(&mut String::new());
    }
    let total: usize = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '[')
                .map(|(x, _)| 100 * y + x)
                .sum::<usize>()
        })
        .sum();
    for row in map.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!()
    }

    total
}

fn move_bot_left_part2(
    start_x: usize,
    start_y: usize,
    map: &mut [Vec<char>],
) -> (usize, usize) {
    let mut x = start_x;
    while x - 1 > 0 {
        x -= 1;
        match map[start_y][x] {
            '#' => break,
            '[' | ']' => continue,
            // If an empty space is reached, move the bot and everything on it's way to the left
            '.' => {
                let row = &mut map[start_y];
                row.remove(x);
                row.insert(start_x, '.');
                return (start_x - 1, start_y);
            }
            _ => unreachable!(),
        }
    }
    (start_x, start_y)
}

fn move_bot_right_part2(
    start_x: usize,
    start_y: usize,
    map: &mut [Vec<char>],
) -> (usize, usize) {
    let most_right_column = map[0].len() - 1;
    let mut x = start_x;
    while x + 1 < most_right_column {
        x += 1;
        match map[start_y][x] {
            '#' => break,
            '[' | ']' => continue,
            // If an empty space is reached, move the bot and everything on it's way to the left
            '.' => {
                let row = &mut map[start_y];
                row.remove(x);
                row.insert(start_x, '.');
                return (start_x + 1, start_y);
            }
            _ => unreachable!(),
        }
    }
    (start_x, start_y)
}

fn move_up_recursive(
    x: usize,
    y: usize,
    map: &mut [Vec<char>],
    old_map: &Vec<Vec<char>>,
) -> bool {
    if old_map[y + 1][x] != '#' {
        map[y][x] = old_map[y + 1][x];
    }
    if y - 1 > 0 {
        return match old_map[y - 1][x] {
            '#' => false,
            '[' => {
                // Avoids doubling the recursion.
                // Only the ']' part will manage recursion if two boxes are directly one above other
                if old_map[y][x] == old_map[y - 1][x] {
                    return true;
                }
                let result = move_up_recursive(x, y - 1, map, old_map)
                    && move_up_recursive(x + 1, y - 1, map, old_map);
                map[y - 1][x] = old_map[y][x];
                result
            }
            ']' => {
                let result = move_up_recursive(x, y - 1, map, old_map)
                    && move_up_recursive(x - 1, y - 1, map, old_map);
                map[y - 1][x] = old_map[y][x];
                result
            }
            '.' => {
                map[y - 1][x] = old_map[y][x];
                true
            }
            _ => unreachable!(),
        };
    }
    false
}

fn move_down_recursive(
    x: usize,
    y: usize,
    map: &mut [Vec<char>],
    old_map: &Vec<Vec<char>>,
) -> bool {
    if old_map[y - 1][x] != '#' {
        map[y][x] = old_map[y - 1][x];
    }
    let bottom_row = map.len() - 1;
    if y + 1 < bottom_row {
        return match old_map[y + 1][x] {
            '#' => {
                println!("for: {}/{}", x, y);
                false
            }
            '[' => {
                let result = move_down_recursive(x, y + 1, map, old_map)
                    && move_down_recursive(x + 1, y + 1, map, old_map);
                map[y + 1][x] = old_map[y][x];
                result
            }
            ']' => {
                let result = move_down_recursive(x, y + 1, map, old_map)
                    && move_down_recursive(x - 1, y + 1, map, old_map);
                map[y + 1][x] = old_map[y][x];
                result
            }
            '.' => {
                map[y + 1][x] = old_map[y][x];
                true
            }
            _ => unreachable!(),
        };
    }
    println!("for: {}/{}", x, y);
    false
}
