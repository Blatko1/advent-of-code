const XMAS: &[&str] = &["X", "M", "A", "S"];

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut xmas_count = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == 'X') {
            // Check North
            if (1..=3).all(|offset| {
                if let Some(y_pos) = y.checked_sub(offset) {
                    return lines[y_pos].get(x..(x + 1)).unwrap()
                        == XMAS[offset];
                };
                false
            }) {
                xmas_count += 1;
            }

            // Check North-East
            if (1..=3).all(|offset| {
                if let Some(y_pos) = y.checked_sub(offset) {
                    if let Some(c) =
                        &lines[y_pos].get((x + offset)..(x + offset + 1))
                    {
                        return *c == XMAS[offset];
                    };
                };
                false
            }) {
                xmas_count += 1;
            }

            // Check East
            if (1..=3).all(|offset| {
                if let Some(c) = &lines[y].get((x + offset)..(x + offset + 1)) {
                    return *c == XMAS[offset];
                };
                false
            }) {
                xmas_count += 1;
            }

            // Check South-East
            if (1..=3).all(|offset| {
                if let Some(y_line) = lines.get(y + offset) {
                    if let Some(c) = y_line.get((x + offset)..(x + offset + 1))
                    {
                        return c == XMAS[offset];
                    };
                }
                false
            }) {
                xmas_count += 1;
            }

            // Check South
            if (1..=3).all(|offset| {
                if let Some(y_line) = lines.get(y + offset) {
                    return y_line.get(x..(x + 1)).unwrap() == XMAS[offset];
                }
                false
            }) {
                xmas_count += 1;
            }

            // Check South-West
            if (1..=3).all(|offset| {
                if let Some(y_line) = lines.get(y + offset) {
                    if let Some(x_pos) = x.checked_sub(offset) {
                        return y_line.get(x_pos..(x_pos + 1)).unwrap()
                            == XMAS[offset];
                    }
                };
                false
            }) {
                xmas_count += 1;
            }

            // Check West
            if (1..=3).all(|offset| {
                if let Some(x_pos) = x.checked_sub(offset) {
                    return lines[y].get(x_pos..(x_pos + 1)).unwrap()
                        == XMAS[offset];
                };
                false
            }) {
                xmas_count += 1;
            }

            // Check North-West
            if (1..=3).all(|offset| {
                if let Some(y_pos) = y.checked_sub(offset) {
                    if let Some(x_pos) = x.checked_sub(offset) {
                        return lines[y_pos].get(x_pos..(x_pos + 1)).unwrap()
                            == XMAS[offset];
                    };
                }
                false
            }) {
                xmas_count += 1;
            }
        }
    }
    xmas_count
}

fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut xmas_count = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == 'A') {
            // Case where above 'A' are 'M' and 'M'
            let check_adjacent_diagonal = |ne: &str, se: &str, sw: &str, nw: &str| -> bool {
                // North-East
                let Some(y_pos) = y.checked_sub(1) else {
                    return false;
                };
                let Some(ne_char) = lines[y_pos].get((x+1)..(x+2)) else {
                    return false;
                };
                if ne_char != ne {
                    return false;
                }

                // South-East
                let Some(y_line) = lines.get(y+1) else {
                    return false;
                };
                let Some(se_char) = y_line.get((x+1)..(x+2)) else {
                    return false;
                };
                if se_char != se {
                    return false;
                }

                // South-West
                let Some(y_line) = lines.get(y+1) else {
                    return false;
                };
                let Some(x_pos) = x.checked_sub(1) else { return false; };

                let Some(sw_char) = y_line.get(x_pos..(x_pos+1)) else {
                    return false;
                };
                if sw_char != sw {
                    return false;
                }

                // North-West
                let Some(y_pos) = y.checked_sub(1) else {
                    return false;
                };
                let Some(x_pos) = x.checked_sub(1) else { return false; };

                let Some(nw_char) = lines[y_pos].get(x_pos..(x_pos+1)) else {
                    return false;
                };
                if nw_char != nw {
                    return false;
                }

                true
            };

            if check_adjacent_diagonal("M", "S", "S", "M") || 
                check_adjacent_diagonal("M", "M", "S", "S") ||
                check_adjacent_diagonal("S", "M", "M", "S") ||
                check_adjacent_diagonal("S", "S", "M", "M") {
                    xmas_count += 1;
                }
        }
    }
    xmas_count
}
