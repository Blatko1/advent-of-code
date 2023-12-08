pub fn part1(input: &str) ->u64 {
    // Parse the input seeds from the 'input'
    let input_seeds: Vec<u64> = input
        .lines()
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    // Filter the input and get an array of map ranges (tuples)
    let maps: Vec<Vec<(u64, u64, u64)>> = input
        .split(':')
        .skip(2)
        .map(|map| {
            let lines = map.lines().filter(|l| contains_digit(l));
            lines
                .map(|l| {
                    let range: Vec<u64> = l
                        .split_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect();
                    (range[0], range[1], range[2])
                })
                .collect::<Vec<(u64, u64, u64)>>()
        })
        .collect();

    let mut min = u64::MAX;
    for seed in input_seeds {
        let mut value = seed;

        // Convert values incrementally following the map order
        'maps: for map in maps.iter() {
            // Check if the value is contained in map ranges
            for range in map {
                let dest_range_start = range.0;
                let src_range_start = range.1;
                let range_len = range.2;

                // If the value is contained in a map range, convert it and continue to next map
                if (src_range_start..(src_range_start + range_len))
                    .contains(&value)
                {
                    value = dest_range_start + value - src_range_start;
                    continue 'maps;
                }
            }
        }
        min = value.min(min);
    }
    min
}

/// Much faster than the brute-force version (4 - 5 seconds in release mode)
pub fn part2(input: &str) -> u64 {
    // Parse the input seed ranges from the 'input'
    let input_seed_ranges_str: &str = input
        .lines()
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap();

    // Split the input seed range numbers into pairs and store as tuples
    let mut seed_ranges: Vec<(u64, u64)> = Vec::new();
    let ranges_split: Vec<u64> = input_seed_ranges_str
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    for i in 0..(ranges_split.len() / 2) {
        seed_ranges.push((ranges_split[i * 2], ranges_split[i * 2 + 1]))
    }

    // Filter the input and get an array of map ranges (tuples)
    let maps: Vec<Vec<(u64, u64, u64)>> = input
        .split(':')
        .skip(2)
        .map(|map| {
            let lines = map.lines().filter(|l| contains_digit(l));
            lines
                .map(|l| {
                    let range: Vec<u64> = l
                        .split_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect();
                    (range[0], range[1], range[2])
                })
                .collect::<Vec<(u64, u64, u64)>>()
        })
        .collect();

    // Find the first existing minimum location
    for location in 0..u64::MAX {
        let mut value = location;
        // Convert the location back into a seed
        for map in maps.iter().rev() {
            for range in map {
                let dest_range_start = range.0;
                let src_range_start = range.1;
                let range_len = range.2;

                // If the value is contained in a map range, convert it and continue to next map
                if (dest_range_start..(dest_range_start + range_len))
                    .contains(&value)
                {
                    value = src_range_start + value - dest_range_start;
                    break;
                }
            }
        }
        if seed_ranges.iter().any(|&(seed_start, seed_range_len)| {
            (seed_start..(seed_start + seed_range_len)).contains(&value)
        }) {
            return location;
        }
    }
    unreachable!("There is no location!")
}

fn contains_digit(l: &str) -> bool {
    l.find(|c: char| c.is_ascii_digit()).is_some()
}
