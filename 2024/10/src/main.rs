fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let map: Vec<Vec<char>> =
        input.lines().map(|row| row.chars().collect()).collect();

    let trailhead_iter = map.iter().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, &c)| c == '0')
            .map(move |(x, _)| (x, y))
    });

    let mut sum = 0;
    for (x, y) in trailhead_iter {
        let mut endings = recursive_hike(&map, 0, x, y);
        endings.sort();
        endings.dedup();
        sum += endings.len();
    }

    sum
}

fn recursive_hike(
    map: &[Vec<char>],
    current_score: usize,
    x: usize,
    y: usize,
) -> Vec<(usize, usize)> {
    //println!("current: {}", current_score);
    if current_score == 9 {
        return vec![(x, y)];
    }

    let mut trail_end_acc = Vec::new();
    // Wrapping around bounds (-1 => usize::MAX)
    if let Some(above_row) = map.get(y.wrapping_sub(1)) {
        let trail_score = above_row[x] as usize - 48;
        if trail_score == current_score + 1 {
            //println!("going up, x: {}, y: {}", x, y-1);
            trail_end_acc.append(&mut recursive_hike(
                map,
                current_score + 1,
                x,
                y - 1,
            ))
        }
    }

    if let Some(below_row) = map.get(y + 1) {
        let trail_score = below_row[x] as usize - 48;
        if trail_score == current_score + 1 {
            //println!("going down, x: {}, y: {}", x, y+1);
            trail_end_acc.append(&mut recursive_hike(
                map,
                current_score + 1,
                x,
                y + 1,
            ))
        }
    }

    if let Some(&left) = map[y].get(x.wrapping_sub(1)) {
        let trail_score = left as usize - 48;
        if trail_score == current_score + 1 {
            //println!("going left, x: {}, y: {}", x-1, y);
            trail_end_acc.append(&mut recursive_hike(
                map,
                current_score + 1,
                x - 1,
                y,
            ))
        }
    }

    if let Some(&right) = map[y].get(x + 1) {
        let trail_score = right as usize - 48;
        if trail_score == current_score + 1 {
            //println!("going right, x: {}, y: {}", x+1, y);
            trail_end_acc.append(&mut recursive_hike(
                map,
                current_score + 1,
                x + 1,
                y,
            ))
        }
    }

    trail_end_acc
}

fn part2(input: &str) -> i32 {
    let map: Vec<Vec<char>> =
        input.lines().map(|row| row.chars().collect()).collect();

    let trailhead_iter = map.iter().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, &c)| c == '0')
            .map(move |(x, _)| (x, y))
    });

    let mut sum = 0;
    for (x, y) in trailhead_iter {
        let pathways = recursive_hike_part2(&map, 0, x, y) + 1;
        sum += pathways;
    }

    sum
}

fn recursive_hike_part2(
    map: &[Vec<char>],
    current_score: usize,
    x: usize,
    y: usize,
) -> i32 {
    //println!("current: {}", current_score);
    if current_score == 9 {
        return 0;
    }

    let mut pathways = 0;
    // Wrapping around bounds (-1 => usize::MAX)
    if let Some(above_row) = map.get(y.wrapping_sub(1)) {
        let trail_score = above_row[x] as usize - 48;
        if trail_score == current_score + 1 {
            //println!("going up, x: {}, y: {}", x, y-1);
            pathways +=
                recursive_hike_part2(map, current_score + 1, x, y - 1) + 1;
        }
    }

    if let Some(below_row) = map.get(y + 1) {
        let trail_score = below_row[x] as usize - 48;
        if trail_score == current_score + 1 {
            //println!("going down, x: {}, y: {}", x, y+1);
            pathways +=
                recursive_hike_part2(map, current_score + 1, x, y + 1) + 1;
        }
    }

    if let Some(&left) = map[y].get(x.wrapping_sub(1)) {
        let trail_score = left as usize - 48;
        if trail_score == current_score + 1 {
            //println!("going left, x: {}, y: {}", x-1, y);
            pathways +=
                recursive_hike_part2(map, current_score + 1, x - 1, y) + 1;
        }
    }

    if let Some(&right) = map[y].get(x + 1) {
        let trail_score = right as usize - 48;
        if trail_score == current_score + 1 {
            //println!("going right, x: {}, y: {}", x+1, y);
            pathways +=
                recursive_hike_part2(map, current_score + 1, x + 1, y) + 1;
        }
    }

    pathways - 1
}
