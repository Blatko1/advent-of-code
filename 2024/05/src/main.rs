use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut page_sum = 0;
    let rules_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let mut page_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (_, [pg_before, pg_after]) in rules_regex.captures_iter(input).map(|c| c.extract()) {
        let before: u32 = pg_before.parse().unwrap();
        let after: u32 = pg_after.parse().unwrap();
        page_map.entry(before).or_default().insert(after);
    }
    for update_line in input.lines().skip_while(|line| !line.trim().is_empty()).skip(1) {
        let updates: Vec<u32> = update_line.split(',').map(|page| page.parse::<u32>().unwrap()).collect();
        let update_count = updates.len();
        let mut is_valid = true;
        for (current, &page) in updates.iter().enumerate().take(update_count-1) {
            let Some(succeding_pages) = page_map.get(&page) else {
                is_valid = false;
                break;
            };
            if updates[current+1..update_count].iter().any(|pg| !succeding_pages.contains(pg)) {
                is_valid = false;
            }
        }
        if is_valid {
            page_sum += updates[update_count/2];
        }
    }
    page_sum
}

fn part2(input: &str) -> u32 {
    let mut page_sum = 0;
    let rules_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let mut page_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (_, [pg_before, pg_after]) in rules_regex.captures_iter(input).map(|c| c.extract()) {
        let before: u32 = pg_before.parse().unwrap();
        let after: u32 = pg_after.parse().unwrap();
        page_map.entry(before).or_default().insert(after);
    }
    for update_line in input.lines().skip_while(|line| !line.trim().is_empty()).skip(1) {
        let mut updates: Vec<u32> = update_line.split(',').map(|page| page.parse::<u32>().unwrap()).collect();
        let update_count = updates.len();
        let mut is_valid = true;
        for (current, &page) in updates.iter().enumerate().take(update_count-1) {
            let Some(succeding_pages) = page_map.get(&page) else {
                is_valid = false;
                break;
            };
            if updates[current+1..update_count].iter().any(|pg| !succeding_pages.contains(pg)) {
                is_valid = false;
            }
        }
        if !is_valid {
            updates.sort_by(|a, b| {
                let Some(succeding_pages) = page_map.get(a) else {
                    return Ordering::Greater;
                };
                match succeding_pages.contains(b) {
                    true => Ordering::Less,
                    false => Ordering::Greater,
                }
            });
            page_sum += updates[update_count/2];
        }
    }
    page_sum
}
