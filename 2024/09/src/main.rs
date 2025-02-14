use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let digits = input.chars().map(|c| c as usize - 48);

    let mut files: VecDeque<(usize, usize)> =
        digits.clone().step_by(2).enumerate().collect();
    let free_spaces = digits.skip(1).step_by(2);

    // ACC can hold only same content (file_id)
    let mut acc_len = 0;
    let mut acc_content = 0;

    let mut position = 0;
    let mut checksum = 0;
    // Iterate over free spaces only since before each free space is a file
    'main: for free_space in free_spaces {
        let Some((file_id, file_lenght)) = files.pop_front() else {
            break;
        };

        for _ in 0..file_lenght {
            checksum += position * file_id;
            position += 1;
        }
        let mut free = free_space;
        while free > 0 {
            // If ACC is empty, put content in it, taking file from the back of disk
            if acc_len == 0 {
                let Some((file_id, file_lenght)) = files.pop_back() else {
                    break 'main;
                };
                acc_len = file_lenght;
                acc_content = file_id;
            }

            acc_len -= 1;
            checksum += position * acc_content;

            position += 1;

            free -= 1;
        }
    }

    while acc_len != 0 {
        acc_len -= 1;
        checksum += position * acc_content;
        position += 1;
    }

    checksum
}

fn part2(input: &str) -> i64 {
    let digits = input.chars().map(|c| (c as usize - 48) as i64);

    let mut files: VecDeque<(i64, i64)> =
        digits.clone().step_by(2).zip(0..).collect();
    let free_spaces = digits.skip(1).step_by(2);

    // ACC can hold only same content (file_id)

    let mut position = 0;
    let mut checksum = 0;
    // Iterate over free spaces only since before each free space is a file
    for free_space in free_spaces {
        let Some((file_lenght, file_id)) = files.pop_front() else {
            break;
        };
        if file_id == -1 {
            //println!("SKIP {}", file_lenght);
            position += file_lenght;
        } else {
            for _ in 0..file_lenght {
                //println!("pos: {}, id: {}", position, file_id);
                checksum += position * file_id;
                position += 1;
            }
        }
        //println!("DOING FREE SPACE");
        let mut free = free_space;
        loop {
            let mut is_found = false;
            for (file_lenght, file_id) in files.iter_mut().rev() {
                let id = *file_id;
                if *file_lenght <= free && id != -1 {
                    for _ in 0..*file_lenght {
                        //println!("pos: {}, id: {}", position, file_id);
                        checksum += position * id;
                        position += 1;
                    }
                    // Marks the file position as free space
                    *file_id = -1;
                    is_found = true;
                    free -= *file_lenght;
                    break;
                }
            }
            if free == 0 {
                break;
            } else if !is_found {
                //println!("SKIP {}", free);
                position += free;
                break;
            }
        }
        //println!("END FREE \n");
    }

    checksum
}
