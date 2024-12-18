use std::collections::VecDeque;

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let bytes: Vec<(usize, usize)> = input.lines().map(|row| {let mut split = row.split(','); let left = split.next().unwrap().parse().unwrap(); let right = split.next().unwrap().parse().unwrap(); (left, right)}).collect();

    let mut map = vec![vec!['.'; WIDTH]; HEIGHT];
    for &(x, y) in bytes.iter().take(1024) {
        map[y][x] = '#';
    }

    let mut min_steps = u32::MAX;
    let mut adjacents = VecDeque::new();
    adjacents.push_back((0, 0, 0));
    while let Some((x, y, acc_steps)) = adjacents.pop_front() {
        if x == (WIDTH - 1) && y == (HEIGHT - 1) {
            min_steps = acc_steps;
            break;
        }

        // Check UP
        if y >= 1 {
            if map[y-1][x] != '#' {
map[y-1][x] = '#';
            adjacents.push_back((x, y-1, acc_steps+1));
            
            }
        }
        // Check DOWN
        if y + 1 < HEIGHT {
            if map[y+1][x] != '#' {
                map[y+1][x] = '#';
            adjacents.push_back((x, y+1, acc_steps+1));
            
            }
        }
        // Check LEFT
        if x >= 1 {
            if map[y][x-1] != '#' {
                map[y][x-1] = '#';
            adjacents.push_back((x-1, y, acc_steps+1));
            
            }
        }
        // Check RIGHT
        if x + 1 < WIDTH {
            if map[y][x+1] != '#' {
                map[y][x+1] = '#';
            adjacents.push_back((x+1, y, acc_steps+1));
            
            }
        }
    }

    min_steps
}

fn part2(input: &str) -> u32 {
    let bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|row| {
            let mut split = row.split(',');
            let left = split.next().unwrap().parse().unwrap();
            let right = split.next().unwrap().parse().unwrap();
            (left, right)
        })
        .collect();

    let mut map = vec![vec!['.'; WIDTH]; HEIGHT];
    for &(x, y) in bytes.iter().take(1024) {
        map[y][x] = '#';
    }

    for i in 1024..3450 {
        let (x, y) = bytes[i];
        map[y][x] = '#';
        let mut bfs_map = map.clone();
        let mut min_steps = u32::MAX;
        let mut adjacents = VecDeque::new();
        adjacents.push_back((0, 0, 0));
        while let Some((x, y, acc_steps)) = adjacents.pop_front() {
            if x == (WIDTH - 1) && y == (HEIGHT - 1) {
                min_steps = acc_steps;
                break;
            }

            // Check UP
            if y >= 1 {
                if bfs_map[y - 1][x] != '#' {
                    bfs_map[y - 1][x] = '#';
                    adjacents.push_back((x, y - 1, acc_steps + 1));
                }
            }
            // Check DOWN
            if y + 1 < HEIGHT {
                if bfs_map[y + 1][x] != '#' {
                    bfs_map[y + 1][x] = '#';
                    adjacents.push_back((x, y + 1, acc_steps + 1));
                }
            }
            // Check LEFT
            if x >= 1 {
                if bfs_map[y][x - 1] != '#' {
                    bfs_map[y][x - 1] = '#';
                    adjacents.push_back((x - 1, y, acc_steps + 1));
                }
            }
            // Check RIGHT
            if x + 1 < WIDTH {
                if bfs_map[y][x + 1] != '#' {
                    bfs_map[y][x + 1] = '#';
                    adjacents.push_back((x + 1, y, acc_steps + 1));
                }
            }
        }
        if min_steps == u32::MAX {
            println!("i: {}, {:?}", i, bytes[i]);
            break;
        }
    }

    todo!()
}
