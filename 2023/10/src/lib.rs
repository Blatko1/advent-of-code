const CONNECTING_FROM_TOP: &[(char, Direction)] = &[
    ('|', Direction::Down),
    ('L', Direction::Right),
    ('J', Direction::Left),
];
const CONNECTING_FROM_BOTTOM: &[(char, Direction)] = &[
    ('|', Direction::Up),
    ('7', Direction::Left),
    ('F', Direction::Right),
];
const CONNECTING_FROM_LEFT: &[(char, Direction)] = &[
    ('-', Direction::Right),
    ('7', Direction::Down),
    ('J', Direction::Up),
];
const CONNECTING_FROM_RIGHT: &[(char, Direction)] = &[
    ('-', Direction::Left),
    ('L', Direction::Up),
    ('F', Direction::Down),
];
const PIPE_POLES: &[&[(char, Direction)]] = &[
    CONNECTING_FROM_BOTTOM,
    CONNECTING_FROM_LEFT,
    CONNECTING_FROM_TOP,
    CONNECTING_FROM_RIGHT,
];

pub fn part1(input: &str) -> u64 {
    let map: Vec<Vec<char>> =
        input.lines().map(|l| l.chars().collect()).collect();

    let (mut x, mut y) = (0, 0);
    'outer: for (i, row) in map.iter().enumerate() {
        for (j, &tile) in row.iter().enumerate() {
            if tile == 'S' {
                y = i;
                x = j;
                break 'outer;
            }
        }
    }
    // The order of elements adjacent to the starting tile: top, right, bottom, left
    let adjacent = [
        (x, y.saturating_sub(1)),
        (x + 1, y),
        (x, y + 1),
        (x.saturating_sub(1), y),
    ];
    // Find the next pipe's leading direction
    let mut direction = Direction::Up;
    // Zip two iterators of pipe poles and adjacent coordinates to compare each pair.
    // First, there is the left adjacent pipe paired with the array of pipes also connecting left.
    // Then, there is the right adjacent pipe paired with the array of pipes also connecting right
    // and so on...
    for (&adj, pipes) in adjacent.iter().zip(PIPE_POLES) {
        let current_pipe = map[adj.1][adj.0];
        if let Some(&(_, dir)) =
            pipes.iter().find(|&&(pipe, _)| current_pipe == pipe)
        {
            x = adj.0;
            y = adj.1;
            direction = dir;
            break;
        }
    }

    // Check the next facing direction and get the pipe at
    // that position and it's leading direction
    let mut steps = 2;
    let mut next_pipe;
    loop {
        match direction {
            Direction::Up => {
                y -= 1;
                next_pipe = map[y][x];
                if next_pipe == 'S' {
                    break;
                }
                let next_direction = CONNECTING_FROM_BOTTOM
                    .iter()
                    .find(|&&(pipe, _)| pipe == next_pipe)
                    .unwrap()
                    .1;
                direction = next_direction;
            }
            Direction::Down => {
                y += 1;
                next_pipe = map[y][x];
                if next_pipe == 'S' {
                    break;
                }
                let next_direction = CONNECTING_FROM_TOP
                    .iter()
                    .find(|&&(pipe, _)| pipe == next_pipe)
                    .unwrap()
                    .1;
                direction = next_direction;
            }
            Direction::Left => {
                x -= 1;
                next_pipe = map[y][x];
                if next_pipe == 'S' {
                    break;
                }
                let next_direction = CONNECTING_FROM_RIGHT
                    .iter()
                    .find(|&&(pipe, _)| pipe == next_pipe)
                    .unwrap()
                    .1;
                direction = next_direction;
            }
            Direction::Right => {
                x += 1;
                next_pipe = map[y][x];
                if next_pipe == 'S' {
                    break;
                }
                let next_direction = CONNECTING_FROM_LEFT
                    .iter()
                    .find(|&&(pipe, _)| pipe == next_pipe)
                    .unwrap()
                    .1;
                direction = next_direction;
            }
        }
        steps += 1;
    }
    steps / 2
}

pub fn part2(input: &str) -> u64 {
    let mut map: Vec<Vec<char>> =
        input.lines().map(|l| l.chars().collect()).collect();
    let line_len = map.first().unwrap().len();
    let mut only_loop_map = vec![vec!['.'; line_len]; map.len()];

    let (mut x, mut y) = (0, 0);
    'outer: for (i, row) in map.iter().enumerate() {
        for (j, &tile) in row.iter().enumerate() {
            if tile == 'S' {
                y = i;
                x = j;
                break 'outer;
            }
        }
    }
    let mut i = 0;
    let mut adjacent_pipes = [Direction::Up; 2];
    // The order of elements adjacent to the starting tile: top, right, bottom, left
    let adjacent = [
        ((x, y.saturating_sub(1)), Direction::Up),
        ((x + 1, y), Direction::Right),
        ((x, y + 1), Direction::Down),
        ((x.saturating_sub(1), y), Direction::Left),
    ];
    // Zip two iterators of pipe poles and adjacent coordinates to compare pipes in each pair.
    // First, there is the left adjacent pipe paired with the array of pipes also connecting left.
    // Then, there is the right adjacent pipe paired with the array of pipes also connecting right
    // and so on...
    for (&(adj, dir), pipes) in adjacent.iter().zip(PIPE_POLES) {
        let current_pipe = &mut map[adj.1][adj.0];
        if pipes.iter().any(|&(pipe, _)| *current_pipe == pipe) {
            adjacent_pipes[i] = dir;
            i += 1;
        }
    }
    let start_tile_pipe = find_start_tile(adjacent_pipes[0], adjacent_pipes[1]);
    only_loop_map[y][x] = start_tile_pipe;
    //println!("adjacents: {:?}, start_tile: {}", adjacent_pipes, start_tile_pipe);

    map[y][x] = '0';
    // The order of elements adjacent to the starting tile: top, right, bottom, left
    let adjacent = [
        (x, y.saturating_sub(1)),
        (x + 1, y),
        (x, y + 1),
        (x.saturating_sub(1), y),
    ];
    // Find the next pipe's leading direction
    let mut direction = Direction::Up;
    // Zip two iterators of pipe poles and adjacent coordinates to compare each pair.
    // First, there is the left adjacent pipe paired with the array of pipes also connecting left.
    // Then, there is the right adjacent pipe paired with the array of pipes also connecting right
    // and so on...
    for (&adj, pipes) in adjacent.iter().zip(PIPE_POLES) {
        let current_pipe = &mut map[adj.1][adj.0];
        if let Some(&(next_pipe, dir)) =
            pipes.iter().find(|&&(pipe, _)| *current_pipe == pipe)
        {
            *current_pipe = '0';
            x = adj.0;
            y = adj.1;
            direction = dir;
            only_loop_map[y][x] = next_pipe;

            break;
        }
    }

    // Check the next facing direction and get the pipe at
    // that position and it's leading direction
    let mut next_pipe;
    loop {
        match direction {
            Direction::Up => {
                y -= 1;
                let pipe = &mut map[y][x];
                next_pipe = *pipe;
                *pipe = '0';
                if next_pipe == '0' {
                    break;
                }
                only_loop_map[y][x] = next_pipe;
                let next_direction = CONNECTING_FROM_BOTTOM
                    .iter()
                    .find(|&&(pipe, _)| pipe == next_pipe)
                    .unwrap()
                    .1;
                direction = next_direction;
            }
            Direction::Down => {
                y += 1;
                let pipe = &mut map[y][x];
                next_pipe = *pipe;
                *pipe = '0';
                if next_pipe == '0' {
                    break;
                }
                only_loop_map[y][x] = next_pipe;
                let next_direction = CONNECTING_FROM_TOP
                    .iter()
                    .find(|&&(pipe, _)| pipe == next_pipe)
                    .unwrap()
                    .1;
                direction = next_direction;
            }
            Direction::Left => {
                x -= 1;
                let pipe = &mut map[y][x];
                next_pipe = *pipe;
                *pipe = '0';
                if next_pipe == '0' {
                    break;
                }
                only_loop_map[y][x] = next_pipe;
                let next_direction = CONNECTING_FROM_RIGHT
                    .iter()
                    .find(|&&(pipe, _)| pipe == next_pipe)
                    .unwrap()
                    .1;
                direction = next_direction;
            }
            Direction::Right => {
                x += 1;
                let pipe = &mut map[y][x];
                next_pipe = *pipe;
                *pipe = '0';
                if next_pipe == '0' {
                    break;
                }
                only_loop_map[y][x] = next_pipe;
                let next_direction = CONNECTING_FROM_LEFT
                    .iter()
                    .find(|&&(pipe, _)| pipe == next_pipe)
                    .unwrap()
                    .1;
                direction = next_direction;
            }
        }
    }
    /*for row in map.iter() {
        for &tile in row.iter() {
            print!("{tile}")
        }
        println!()
    }
    println!();
    for row in only_loop_map.iter() {
        for &tile in row.iter() {
            print!("{tile}")
        }
        println!()
    }
    let mut enclosed_map = only_loop_map.clone();*/

    let mut enclosed_tiles = 0;
    for (_y, (row, loop_row)) in map.iter().zip(only_loop_map).enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == '0' {
                continue;
            }
            let hit_count = side_hit_count(&loop_row[(x + 1)..]);
            //println!("tile: {}, hits: {}", tile, hit_count);
            if hit_count % 2 != 0 {
                enclosed_tiles += 1;
                //enclosed_map[_y][x] = '#';
            }
        }
    }
    /*println!();
    for row in enclosed_map.iter() {
        for &tile in row.iter() {
            print!("{tile}")
        }
        println!()
    }*/
    enclosed_tiles
}

const BENDING_PIPES: &[char] = &['L', 'F', '7', 'J'];

fn side_hit_count(tiles: &[char]) -> u64 {
    let mut hits = 0;
    let mut last_bending_pipe = '0';
    for &tile in tiles {
        if BENDING_PIPES.contains(&tile) {
            match (last_bending_pipe, tile) {
                ('L', 'J') => {
                    hits -= 1;
                    continue;
                }
                ('F', '7') => {
                    hits -= 1;
                    continue;
                }
                ('L', '7') => continue,
                ('F', 'J') => continue,
                _ => (),
            }
            hits += 1;
            last_bending_pipe = tile;
        } else if tile == '|' {
            hits += 1;
        }
    }
    hits
}

fn find_start_tile(
    first_adjacent: Direction,
    last_adjacent: Direction,
) -> char {
    match (first_adjacent, last_adjacent) {
        (Direction::Up, Direction::Left) => 'L',
        (Direction::Up, Direction::Down) => '|',
        (Direction::Up, Direction::Right) => 'J',
        (Direction::Right, Direction::Down) => 'F',
        (Direction::Right, Direction::Left) => '-',
        (Direction::Down, Direction::Left) => '7',
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[test]
fn side_count_hit_test() {
    let input = &[
        'L', '7', '0', '0', '0', 'L', 'J', 'F', '7', 'F', '-', '7', 'L', '7',
        '0',
    ];
    assert_eq!(side_hit_count(input), 2);
}
