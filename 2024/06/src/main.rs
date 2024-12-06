use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut map: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let start_pos = || -> (usize, usize) {
        for (y, row) in map.iter().enumerate() {
            if let Some(x) = row.iter().position(|c| *c == '^') {
                return (x, y);
            };
        }
        unreachable!()
    };

    let (mut x, mut y) = start_pos();
    // X-positive points right, Y-positive points up (so subtracting in calculation)
    let mut direction = (0, 1);
    map[y][x] = 'X';

    loop {
        // If negative, it will wrap around bounds
        let new_x_pos = (x as i32 + direction.0) as usize;
        let new_y_pos = (y as i32 - direction.1) as usize;
        if let Some(row) = map.get_mut(new_y_pos) {
            if let Some(next_field) = row.get_mut(new_x_pos) {
                if *next_field == '#' {
                    let dir_0 = direction.0;
                    direction.0 = direction.1;
                    direction.1 = -dir_0;
                } else {
                    *next_field = 'X';
                    x = new_x_pos;
                    y = new_y_pos;
                }
                continue;
            }
        }
        break;
    }

    let mut count = 0;
    for row in map.iter() {
        count += row.iter().filter(|&&c| c == 'X').count();
    }

    count
}

const OBSTRUCTION: char = 'O';
const EMPTY: char = '.';

fn part2(input: &str) -> u32 {
    //let original = input.to_owned();
    //let mut modified = input.to_owned();
    let mut map: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let start_pos = || -> (usize, usize) {
        for (y, row) in map.iter().enumerate() {
            if let Some(x) = row.iter().position(|c| *c == '^') {
                return (x, y);
            };
        }
        unreachable!()
    };

    let (start_x, start_y) = start_pos();
    let start_direction = (0, 1);
    let (mut x, mut y) = (start_x, start_y);
    // X-positive points right, Y-positive points up (so subtracting in calculation)
    let mut direction = start_direction;

    loop {
        // If negative, it will wrap around bounds
        let new_x_pos = (x as i32 + direction.0) as usize;
        let new_y_pos = (y as i32 - direction.1) as usize;
        if let Some(row) = map.get_mut(new_y_pos) {
            if let Some(next_field) = row.get_mut(new_x_pos) {
                if *next_field == '#' {
                    let dir_0 = direction.0;
                    direction.0 = direction.1;
                    direction.1 = -dir_0;
                } else {
                    *next_field = 'X';
                    x = new_x_pos;
                    y = new_y_pos;
                }
                continue;
            }
        }
        break;
    }

    // Get position of every 'X' trail on the path
    let obstruction_fields: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'X')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let mut loops = 0;
    for (obstruction_x, obstruction_y) in obstruction_fields {
        map[obstruction_y][obstruction_x] = OBSTRUCTION;

        //modified.replace_range((obstruction_y*12 + obstruction_x)..(obstruction_y*12 + obstruction_x+1), "O");
        //std::fs::write("2024/06/input_test.txt", &modified).unwrap();            
        
        let (mut x, mut y) = (start_x, start_y);
        let mut direction = start_direction;
        let mut encountered_obstacles = HashSet::new();
        loop {
            // If negative, it will wrap around bounds
            let new_x_pos = (x as i32 + direction.0) as usize;
            let new_y_pos = (y as i32 - direction.1) as usize;
            if let Some(row) = map.get_mut(new_y_pos) {
                if let Some(next_field) = row.get_mut(new_x_pos) {
                    if ['#', OBSTRUCTION].contains(next_field) {
                        if !encountered_obstacles.insert(((new_x_pos, new_y_pos), direction)) {
                            //println!("DID!!!!");
                            loops += 1;
                            break;
                        }
                        let dir_0 = direction.0;
                        direction.0 = direction.1;
                        direction.1 = -dir_0;
                    } else {
                        x = new_x_pos;
                        y = new_y_pos;
                        //modified.replace_range((y*12 + x)..(y*12 + x+1), "X");
                        //std::fs::write("2024/06/input_test.txt", &modified).unwrap();
                        //std::thread::sleep(std::time::Duration::from_millis(300));
                    }
                    continue;
                }
            }
            break;
        }

        //modified = original.clone();
        //std::fs::write("2024/06/input_test.txt", &modified).unwrap();  
        map[obstruction_y][obstruction_x] = EMPTY;
    }

    loops
}
