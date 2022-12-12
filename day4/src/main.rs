use std::cmp;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = get_puzzle_input().expect("Line Read Error").lines();
    let res = lines.fold(0, complete_overlap_fold);
    let lines = get_puzzle_input().expect("Line Read Error").lines();
    let res2 = lines.fold(0, all_overlap_fold);
    println!("Complete Overlap: {res}; All Overlap: {res2}");
}

fn all_overlap_fold(acc: usize, input: Result<String, std::io::Error>) -> usize {
    let ((min1, max1), (min2, max2)) = parse_string(input.expect("No line Error in fold_fun!"));
    print!("{min1}..{max1}; {min2}..{max2}");
    if max1 >= min2 && max2 >= min1 {
                println!(" inc");
        return acc + 1;
    }
    println!("");
    return acc;
}

fn complete_overlap_fold(acc: usize, input: Result<String, std::io::Error>) -> usize {
    let ((min1, max1), (min2, max2)) = parse_string(input.expect("No line Error in fold_fun!"));
    if max1 >= min2 || max2 >= min1 {
        if min1 < min2 {
            if max2 <= max1 {
                return acc + 1;
            }
        } else if min1 > min2 {
            if max1 <= max2 {
                return acc + 1;
            }
        } else {
            return acc + 1;
        }
    }
    return acc;
}

fn parse_string(line: String) -> ((usize, usize), (usize, usize)) {
    let (elf1, elf2) = line
        .split_once(',')
        .expect("Couldn't find ',' in the String!");
    let (min1, max1) = elf1
        .split_once('-')
        .expect("Couldn't find '-' in the String from elf1!");
    let (min2, max2) = elf2
        .split_once('-')
        .expect("Couldn't find '-' in the String from elf2!");
    (
        (
            min1.parse::<usize>().unwrap(),
            max1.parse::<usize>().unwrap(),
        ),
        (
            min2.parse::<usize>().unwrap(),
            max2.parse::<usize>().unwrap(),
        ),
    )
}

fn get_puzzle_input() -> Result<BufReader<File>, std::io::Error> {
    let vars: Vec<String> = env::args().collect();
    let path = &vars[1];
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    return Ok(buffered);
}
