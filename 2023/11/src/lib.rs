pub fn part1(input: &str) -> u64 {
    let mut map: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let original_row_len = map.first().unwrap().len();
    let empty_row = vec!['x'; original_row_len];

    // Find which map rows are empty (only '.') and add another
    // empty_row adjacent to that map row
    let mut offset = 0;
    'outer: for i in 0..map.len() {
        for &tile in map[offset + i].iter() {
            if tile == '#' {
                continue 'outer;
            }
        }
        map.insert(offset + i, empty_row.clone());
        offset += 1;
    }

    // Find which map columns are empty (only '.') and add another
    // column adjacent to that map row (by adding a new '.' at that index)
    let mut offset = 0;
    'outer: for i in 0..original_row_len {
        for row in map.iter() {
            if row[offset + i] == '#' {
                continue 'outer;
            }
        }
        for row in map.iter_mut() {
            row.insert(offset + i, 'x');
        }
        offset += 1;
    }

    // Find coordinates of each galaxy
    let mut galaxies = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == '#' {
                galaxies.push((x as i32, y as i32));
            }
        }
    }

    /*for row in map.iter() {
        for tile in row.iter() {
            print!("{tile}");
        }
        println!()
    }*/

    let mut distance_sum = 0;
    let galaxy_count = galaxies.len();
    for i in 0..(galaxy_count - 1) {
        let first = galaxies[i];
        for second in galaxies.iter().skip(i + 1) {
            let distance =
                (first.0 - second.0).abs() + (first.1 - second.1).abs();
            distance_sum += distance;
        }
    }
    distance_sum as u64
}

pub fn part2(input: &str) -> u64 {
    let mut map: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let row_len = map.first().unwrap().len();

    // Find which map rows are empty (only '.') and add another
    // empty_row adjacent to that map row
    'outer: for row in map.iter_mut() {
        for &tile in row.iter() {
            if tile == '#' {
                continue 'outer;
            }
        }
        row.fill('x');
    }

    // Find which map columns are empty (only '.') and add another
    // column adjacent to that map row (by adding a new '.' at that index)
    'outer: for i in 0..row_len {
        for row in map.iter() {
            if row[i] == '#' {
                continue 'outer;
            }
        }
        for row in map.iter_mut() {
            row[i] = 'x';
        }
    }

    // Find coordinates of each galaxy
    let additional_space_width = 1_000_000i64;
    let mut galaxies = Vec::new();
    for y in 0..map.len() {
        let row = &map[y];
        for x in 0..row_len {
            let tile = row[x];
            if tile == '#' {
                let additional_x_spaces =
                    row[0..x].iter().filter(|&&tile| tile == 'x').count();
                let additional_x_distance =
                    additional_x_spaces as i64 * additional_space_width;
                let mut additional_y_spaces = 0;
                for row in map[0..y].iter() {
                    if row[x] == 'x' {
                        additional_y_spaces += 1;
                    }
                }

                let x = 1 + x - additional_x_spaces;
                let y = 1 + y - additional_y_spaces;
                let additional_y_distance =
                    additional_y_spaces as i64 * additional_space_width;

                galaxies.push((
                    x as i64 + additional_x_distance,
                    y as i64 + additional_y_distance,
                ));
            }
        }
    }

    /*for row in map.iter() {
        for tile in row.iter() {
            print!("{tile}");
        }
        println!()
    }*/

    let mut distance_sum = 0;
    let galaxy_count = galaxies.len();
    for i in 0..(galaxy_count - 1) {
        let first = galaxies[i];
        for second in galaxies.iter().skip(i + 1) {
            let distance =
                (first.0 - second.0).abs() + (first.1 - second.1).abs();
            distance_sum += distance;
        }
    }
    distance_sum as u64
}
