use std::{collections::HashMap, u32, usize};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    //println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let map: Vec<Vec<char>> =
        input.lines().map(|row| row.chars().collect()).collect();
    let map_width = map.first().unwrap().len();
    let map_height = map.len();

    let mut start = (0, 0);
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == 'S') {
            start = (x, y);
        }
    }

    let mut unvisited_fields: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '.')
                .map(move |(x, _)| (x, y))
        })
        .collect();
    unvisited_fields.sort();

    let mut encountered_fileds = vec![start];

    let mut visited_fields = Vec::with_capacity(unvisited_fields.len());
    visited_fields.push(start);

    let mut field_distances: HashMap<
        (usize, usize),
        (u32, Option<(usize, usize)>),
    > = unvisited_fields
        .iter()
        .map(|&pos| (pos, u32::MAX, None))
        .collect();
    field_distances.insert(start, (0, None));

    let dir = (1, 0);
    loop {
        let min_points_field = encountered_fileds
            .iter()
            .min_by(|a, b| field_distances[a].cmp(&field_distances[b]))
            .unwrap();

        // Check adjacent field in direction forward
        if map[y + dir.1][x + dir.0] == '.' {
            let pos = (x + dir.0, y + dir.1);
            if unvisited_fields.binary_search(&pos).is_ok() {
                let old = *field_distances
                    .get_mut(&pos)
                    .replace(&mut (1, Some((x, y))))
                    .unwrap();
                if old.1.is_none() {
                    encountered_fileds.push(pos);
                }
            }
        }
        // Check adjacent field in direction rotating left
        if map[y + dir.0][x - dir.1] == '.' {
            let pos = (x - dir.1, y + dir.0);
            if unvisited_fields.binary_search(&pos).is_ok() {
                let old = *field_distances
                    .get_mut(&pos)
                    .replace(&mut (1001, Some((x, y))))
                    .unwrap();
                if old.1.is_none() {
                    encountered_fileds.push(pos);
                }
            }
        }
        // Check adjacent field in direction rotating right
        if map[y - dir.0][x + dir.1] == '.' {
            let pos = (x + dir.1, y - dir.0);
            if unvisited_fields.binary_search(&pos).is_ok() {
                let old = *field_distances
                    .get_mut(&pos)
                    .replace(&mut (1001, Some((x, y))))
                    .unwrap();
                if old.1.is_none() {
                    encountered_fileds.push(pos);
                }
            }
        }
    }

    //let lowest_points_path = DfsBestPath::new(map_width, map_height).search(map);

    todo!()
}

/*struct DfsBestPath {
    map_width: usize,
    map_height: usize,
    min_points: u32
}

impl DfsBestPath {
    fn new(map_width: usize, map_height: usize) -> Self {
        Self {
            map_width,
            map_height,
            min_points: u32::MAX
        }
    }

    fn search(mut self, map: Vec<Vec<char>>) -> u32 {
        let mut start = (0, 0);
        for (y, row) in map.iter().enumerate() {
            if let Some(x) = row.iter().position(|&c| c == 'S') {
                start = (x, y);
            }
        }
        self.recursive_search(map, start.0, start.1, Direction::East, 0);
        self.min_points
    }

    fn recursive_search(&mut self, mut map: Vec<Vec<char>>, x: usize, y: usize, direction: Direction, acc_points: u32) {
        //println!("curent: {}/{}, dir: {:?}, acc: {}", x, y, direction, acc_points);
        if map[y][x] == 'E' {
            self.min_points = self.min_points.min(acc_points);
            return;
        }
        map[y][x] = '#';
        /*for row in map.iter() {
            for field in row.iter() {
                print!("{}", field);
            }
            println!()
        }
        std::io::stdin().read_line(&mut String::new());*/
        if y - 1 > 0 {
            if map[y-1][x] != '#' {
                let new_dir = Direction::North;
                let add = if direction == new_dir {
                    1
                } else {
                    1001
                };
                self.recursive_search(map.clone(), x, y-1, new_dir, acc_points + add);
            }
        }
        if y + 1 < (self.map_height - 1) {
            if map[y+1][x] != '#' {
                let new_dir = Direction::South;
                let add = if direction == new_dir {
                    1
                } else {
                    1001
                };
                self.recursive_search(map.clone(), x, y+1, new_dir, acc_points + add);
            }
        }
        if x - 1 > 0 {
            if map[y][x-1] != '#' {
                let new_dir = Direction::West;
                let add = if direction == new_dir {
                    1
                } else {
                    1001
                };
                self.recursive_search(map.clone(), x-1, y, new_dir, acc_points + add);
            }
        }
        if x + 1 < (self.map_width - 1) {
            if map[y][x+1] != '#' {
                let new_dir = Direction::East;
                let add = if direction == new_dir {
                    1
                } else {
                    1001
                };
                self.recursive_search(map.clone(), x+1, y, new_dir, acc_points + add);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West
}*/
