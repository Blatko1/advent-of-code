fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut map: Vec<Vec<(usize, usize, char)>> = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars().enumerate().map(|(x, c)| (x, y, c)).collect()
        })
        .collect();
    let max_width = map.first().unwrap().len() as i32;
    let max_height = map.len() as i32;

    let mut total = 0;
    while let Some((start_x, start_y, plot)) = find_next_field(&map) {
        let plot_fields =
            Dfs::new(&mut map, plot).search(start_x as i32, start_y as i32);
        /*println!("len: {}", plot_fields.len());
        for row in map.iter() {
            for field in row.iter() {
                print!("{}", field.2);
            }
            println!();
        }*/

        let plot_fiels_count = plot_fields.len();
        let mut fences = 0;
        let plot = plot.to_ascii_lowercase();
        for (x, y) in plot_fields {
            if y as i32 - 1 >= 0 {
                let adjacent_plot = map[y - 1][x].2;
                if adjacent_plot != plot {
                    fences += 1;
                }
            } else {
                fences += 1;
            }
            if y as i32 + 1 < max_height {
                let adjacent_plot = map[y + 1][x].2;
                if adjacent_plot != plot {
                    fences += 1;
                }
            } else {
                fences += 1;
            }
            if x as i32 - 1 >= 0 {
                let adjacent_plot = map[y][x - 1].2;
                if adjacent_plot != plot {
                    fences += 1;
                }
            } else {
                fences += 1;
            }
            if x as i32 + 1 < max_width {
                let adjacent_plot = map[y][x + 1].2;
                if adjacent_plot != plot {
                    fences += 1;
                }
            } else {
                fences += 1;
            }
        }
        total += fences * plot_fiels_count;
        //println!("fences: {}\n", fences);
    }

    total
}

fn part2(input: &str) -> usize {
    let mut map: Vec<Vec<(usize, usize, char)>> = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars().enumerate().map(|(x, c)| (x, y, c)).collect()
        })
        .collect();
    let max_width = map.first().unwrap().len() as i32;
    let max_height = map.len() as i32;

    let mut total = 0;
    while let Some((start_x, start_y, plot)) = find_next_field(&map) {
        let plot_fields =
            Dfs::new(&mut map, plot).search(start_x as i32, start_y as i32);

        let plot_fiels_count = plot_fields.len();
        let mut fences = Vec::new();
        let plot = plot.to_ascii_lowercase();
        for (x, y) in plot_fields {
            if y as i32 - 1 >= 0 {
                let adjacent_plot = map[y - 1][x].2;
                if adjacent_plot != plot {
                    fences.push((
                        x as i32,
                        y as i32 - 1,
                        Orientation::Horizontal,
                        AttachedTo::Down,
                    ));
                }
            } else {
                fences.push((
                    x as i32,
                    y as i32 - 1,
                    Orientation::Horizontal,
                    AttachedTo::Down,
                ));
            }
            if y as i32 + 1 < max_height {
                let adjacent_plot = map[y + 1][x].2;
                if adjacent_plot != plot {
                    fences.push((
                        x as i32,
                        y as i32 + 1,
                        Orientation::Horizontal,
                        AttachedTo::Up,
                    ));
                }
            } else {
                fences.push((
                    x as i32,
                    y as i32 + 1,
                    Orientation::Horizontal,
                    AttachedTo::Up,
                ));
            }
            if x as i32 - 1 >= 0 {
                let adjacent_plot = map[y][x - 1].2;
                if adjacent_plot != plot {
                    fences.push((
                        x as i32 - 1,
                        y as i32,
                        Orientation::Vertical,
                        AttachedTo::Right,
                    ));
                }
            } else {
                fences.push((
                    x as i32 - 1,
                    y as i32,
                    Orientation::Vertical,
                    AttachedTo::Right,
                ));
            }
            if x as i32 + 1 < max_width {
                let adjacent_plot = map[y][x + 1].2;
                if adjacent_plot != plot {
                    fences.push((
                        x as i32 + 1,
                        y as i32,
                        Orientation::Vertical,
                        AttachedTo::Left,
                    ));
                }
            } else {
                fences.push((
                    x as i32 + 1,
                    y as i32,
                    Orientation::Vertical,
                    AttachedTo::Right,
                ));
            }
        }
        let mut merged_fences = 0;
        while !fences.is_empty() {
            let (x, y, orientation, attached_to) = fences.pop().unwrap();
            match orientation {
                Orientation::Horizontal => {
                    //println!("remove horizontal at {}/{}", x, y);
                    remove_horizonal_dir(&mut fences, x, y, -1, attached_to);
                    remove_horizonal_dir(&mut fences, x, y, 1, attached_to);
                }
                Orientation::Vertical => {
                    //println!("remove vertical at {}/{}", x, y);
                    remove_vertical_dir(&mut fences, x, y, -1, attached_to);
                    remove_vertical_dir(&mut fences, x, y, 1, attached_to);
                }
            }
            merged_fences += 1;
            //println!();
        }
        //println!("merged: {}", merged_fences);

        total += merged_fences * plot_fiels_count;
    }

    total
}

fn remove_horizonal_dir(
    fences: &mut Vec<(i32, i32, Orientation, AttachedTo)>,
    x: i32,
    y: i32,
    direction: i32,
    attached_to: AttachedTo,
) {
    if let Some(idx) = fences.iter().position(|f| {
        f == &(x + direction, y, Orientation::Horizontal, attached_to)
    }) {
        //println!("removed at {}/{}!", x+direction, y);
        fences.remove(idx);
        remove_horizonal_dir(fences, x + direction, y, direction, attached_to);
    }
}

fn remove_vertical_dir(
    fences: &mut Vec<(i32, i32, Orientation, AttachedTo)>,
    x: i32,
    y: i32,
    direction: i32,
    attached_to: AttachedTo,
) {
    if let Some(idx) = fences.iter().position(|f| {
        f == &(x, y + direction, Orientation::Vertical, attached_to)
    }) {
        //println!("removed at {}/{}!", x, y+direction);
        fences.remove(idx);
        remove_vertical_dir(fences, x, y + direction, direction, attached_to);
    }
}

struct Dfs<'a> {
    max_width: i32,
    max_height: i32,
    map: &'a mut [Vec<(usize, usize, char)>],
    plot: char,
}

impl<'a> Dfs<'a> {
    fn new(map: &'a mut [Vec<(usize, usize, char)>], plot: char) -> Self {
        let max_width = map.first().unwrap().len() as i32;
        let max_height = map.len() as i32;
        Self {
            max_width,
            max_height,
            map,
            plot,
        }
    }
    fn search(mut self, start_x: i32, start_y: i32) -> Vec<(usize, usize)> {
        assert!(
            (0..self.max_width).contains(&start_x)
                && (0..self.max_height).contains(&start_y)
        );
        self.recursive_step(start_x, start_y)
    }
    fn recursive_step(&mut self, x: i32, y: i32) -> Vec<(usize, usize)> {
        let field = &mut self.map[y as usize][x as usize].2;
        if *field != self.plot {
            return vec![];
        }
        // While stepping, immediately replace the plot with lowercase to mark it
        *field = self.plot.to_ascii_lowercase();

        let mut fields = vec![(x as usize, y as usize)];
        if y - 1 >= 0 {
            fields.append(&mut self.recursive_step(x, y - 1));
        }
        if y + 1 < self.max_height {
            fields.append(&mut self.recursive_step(x, y + 1));
        }
        if x - 1 >= 0 {
            fields.append(&mut self.recursive_step(x - 1, y));
        }
        if x + 1 < self.max_width {
            fields.append(&mut self.recursive_step(x + 1, y));
        }

        fields
    }
}

fn find_next_field(
    map: &[Vec<(usize, usize, char)>],
) -> Option<(usize, usize, char)> {
    for row in map.iter() {
        if let Some(&field) =
            row.iter().find(|(_, _, plot)| !plot.is_ascii_lowercase())
        {
            return Some(field);
        }
    }
    None
}

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AttachedTo {
    Left,
    Right,
    Up,
    Down,
}
