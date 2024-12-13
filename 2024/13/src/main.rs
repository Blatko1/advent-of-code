use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let regex = Regex::new(r"A: X\+(\d+), Y+\+(\d+)[\s\S]+?B: X\+(\d+), Y\+(\d+)[\s\S]+?X=(\d+), Y=(\d+)").unwrap();
    let mut total = 0;
    for (_, [xa, ya, xb, yb, total_x, total_y]) in
        regex.captures_iter(input).map(|c| c.extract())
    {
        // How much each buttons moves in x/y direction
        let xa: f32 = xa.parse().unwrap();
        let ya: f32 = ya.parse().unwrap();
        let xb: f32 = xb.parse().unwrap();
        let yb: f32 = yb.parse().unwrap();

        // Final position of the claw
        let total_x: f32 = total_x.parse().unwrap();
        let total_y: f32 = total_y.parse().unwrap();

        let b_clicks = (total_y * xa - ya * total_x) / (yb * xa - xb * ya);
        let a_clicks = (total_x - b_clicks * xb) / xa;

        if a_clicks < 0.0
            || b_clicks < 0.0
            || a_clicks.fract() != 0.0
            || b_clicks.fract() != 0.0
        {
            continue;
        }

        //println!("how much: A: {}, B: {}", a_clicks, b_clicks);
        total += a_clicks as u32 * 3 + b_clicks as u32;
    }

    total
}

fn part2(input: &str) -> u64 {
    let regex = Regex::new(r"A: X\+(\d+), Y+\+(\d+)[\s\S]+?B: X\+(\d+), Y\+(\d+)[\s\S]+?X=(\d+), Y=(\d+)").unwrap();
    let mut total = 0;
    for (_, [xa, ya, xb, yb, total_x, total_y]) in
        regex.captures_iter(input).map(|c| c.extract())
    {
        // How much each buttons moves in x/y direction
        let xa: f64 = xa.parse().unwrap();
        let ya: f64 = ya.parse().unwrap();
        let xb: f64 = xb.parse().unwrap();
        let yb: f64 = yb.parse().unwrap();

        // Final position of the claw
        let total_x = total_x.parse::<f64>().unwrap() + 10_000_000_000_000.0;
        let total_y = total_y.parse::<f64>().unwrap() + 10_000_000_000_000.0;

        let b_clicks = (total_y * xa - ya * total_x) / (yb * xa - xb * ya);
        let a_clicks = (total_x - b_clicks * xb) / xa;

        if a_clicks < 0.0
            || b_clicks < 0.0
            || a_clicks.fract() != 0.0
            || b_clicks.fract() != 0.0
        {
            continue;
        }

        //println!("how much: A: {}, B: {}", a_clicks, b_clicks);
        total += a_clicks as u64 * 3 + b_clicks as u64;
    }

    total
}
