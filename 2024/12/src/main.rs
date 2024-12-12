fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    //println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut map: Vec<Vec<(usize, usize, char)>> = input.lines().enumerate().map(|(y, row)| row.chars().enumerate().map(|(x, c)| (x, y, c)).collect()).collect();

    while let Some((x, y, plot)) = find_next_field(&map) {
        let plot_fields = Dfs::new(&mut map, plot).search(x as i32, y as i32);
        println!("len: {}", plot_fields.len());
        for row in map.iter() {
            for field in row.iter() {
                print!("{}", field.2);
            }
            println!();
        }
        println!()
    }

    todo!()
}

struct Dfs<'a> {
    max_width: i32,
    max_height: i32,
    map: &'a mut [Vec<(usize, usize, char)>],
    plot: char
}

impl<'a> Dfs<'a> {
    fn new(map: &'a mut [Vec<(usize, usize, char)>], plot: char) -> Self {
        let max_width = map.first().unwrap().len() as i32;
        let max_height = map.len() as i32;
        Self {
            max_width,
            max_height,
            map,
            plot
        }
    }
    fn search(mut self, start_x: i32, start_y: i32) -> Vec<(usize, usize)> {
        assert!((0..self.max_width).contains(&start_x) && (0..self.max_height).contains(&start_y));
        self.step(start_x, start_y)
    }
    fn step(&mut self, x: i32, y: i32) -> Vec<(usize, usize)> {
        let field = &mut self.map[y as usize][x as usize].2;
        if *field != self.plot {
            return vec![]
        }
        // While stepping, immediately replace the plot with empty space '.'
        *field = '.';

        let mut fields = vec![(x as usize, y as usize)];
        if y - 1 >= 0 {
            fields.append(&mut self.step(x, y-1));
        }
        if y + 1 < self.max_height {
            fields.append(&mut self.step(x, y+1));
        }
        if x - 1 >= 0 {
            fields.append(&mut self.step(x-1, y));
        }
        if x + 1 < self.max_width {
            fields.append(&mut self.step(x+1, y));
        }
        
        return fields
    }
}

fn find_next_field(map: &[Vec<(usize, usize, char)>]) -> Option<(usize, usize, char)> {
    for row in map.iter() {
        if let Some(&field) = row.iter().find(|(_, _, plot)| *plot != '.') {
            return Some(field)
        }
    }
    None
}