fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input);
    let part1_for_loop = part1_for_loop(input);
    assert_eq!(part1, part1_for_loop);
    println!("Part 1 result: {}", part1);
    println!("Part 2 result: {}", part2_brute_force(input));
}

fn are_levels_safe(levels: &[i32]) -> Result<(), usize> {
    let increasing = levels[0] < levels[1];
    let mut levels_iter = levels.iter().enumerate().peekable();
    while let Some((idx, current)) = levels_iter.next() {
        if let Some((_, &next)) = levels_iter.peek() {
            let diff = next - current;
            if (increasing && (diff <= 0 || diff > 3))
                || (!increasing && (diff >= 0 || diff < -3))
            {
                return Err(idx);
            }
        }
    }
    Ok(())
}

fn part1(input: &str) -> u32 {
    let mut safe_reports = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();
        if are_levels_safe(&levels).is_ok() {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn part1_for_loop(input: &str) -> u32 {
    let mut safe_reports = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();
        let level_count = levels.len();
        let mut safe = true;
        let increasing = levels[0] < levels[1];
        for (current, next) in levels[0..level_count - 1]
            .iter()
            .zip(levels[1..level_count].iter())
        {
            let diff = next - current;
            if (increasing && (diff <= 0 || diff > 3))
                || (!increasing && (diff >= 0 || diff < -3))
            {
                safe = false;
                break;
            }
        }
        if safe {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn part2(input: &str) -> u32{
    let mut safe_reports = 0;
    for line in input.lines() {
        let mut levels: Vec<i32> = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();
        let level_count = levels.len();

        match are_levels_safe(&levels) {
            Ok(_) => safe_reports += 1,
            Err(level_idx) => {
                let mut level_idx = level_idx;
                for a in 0..(3).min(level_count -level_idx) {
                    //println!("{}", a);
                   let mut levels = levels.clone();
                   //println!("removed: {}", levels.get(level_idx).unwrap());
                   levels.remove(level_idx);
                    if are_levels_safe(&levels).is_ok() {
                        safe_reports += 1;
                        break;
                    } 
                    level_idx += 1;
                }
                
            }
        }
        //println!("");
    }
    safe_reports
}

fn part2_brute_force(input: &str) -> i32 {
    let mut safe_reports = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();

        if are_levels_safe(&levels).is_ok() {
            safe_reports += 1;
            continue;
        }

        for idx in 0..levels.len() {
            let mut dampened_levels = levels.clone();
            dampened_levels.remove(idx);
            if are_levels_safe(&dampened_levels).is_ok() {
                safe_reports += 1;
                break;
            }
        }
    }
    safe_reports
}