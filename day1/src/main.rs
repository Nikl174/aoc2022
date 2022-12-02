use std::cmp;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let vars: Vec<String> = env::args().collect();
    let path = &vars[1];
    let input = File::open(path).expect("File open error from path {path}");
    let buffered = BufReader::new(input);
    let result = buffered.lines().fold(((0, 0, 0), 0), biggest_three_elfs);

    println!(
        "Maximum Inventory (part1): {}\n 3 biggest inventories (part2) {}",
        result.0.1,
        result.0 .0 + result.0 .1 + result.0 .2
        );
    return Ok(());
}

// Folding function for calculating the current backpack and maximum
fn biggest_elf(acc: (u32, u32), value: Result<String, std::io::Error>) -> (u32, u32) {
    let (max, current) = acc;
    let val_as_int: u32 = match value.expect("Input was no string in biggest_elf!").parse() {
        Ok(int) => int,
        Err(_) => return (cmp::max(max, current), 0),
    };
    return (max, current + val_as_int);
}

fn biggest_three_elfs(
    acc: ((u32, u32, u32), u32),
    line: Result<String, std::io::Error>,
) -> ((u32, u32, u32), u32) {
    let (max_tupel, current) = acc;
    let val_as_int: u32 = match line.expect("Input was no string in biggest_elf!").parse() {
        Ok(int) => int,
        Err(_) => return (insert_to_max_tupel(max_tupel, current), 0),
    };
    return (max_tupel, current + val_as_int);
}

fn insert_to_max_tupel(max_tupel: (u32, u32, u32), new_value: u32) -> (u32, u32, u32) {
    let (first_max, second_max, third_max) = max_tupel;
    if new_value >= third_max {
        if new_value >= second_max {
            if new_value >= first_max {
                return (new_value, first_max, second_max);
            }
            return (first_max, new_value, second_max);
        }
        return (first_max, second_max, new_value);
    } else {
        return max_tupel;
    }
}

