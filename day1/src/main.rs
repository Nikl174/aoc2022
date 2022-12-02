use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};


fn main() -> Result<(), Error> {
    let path = "input.in";

    let input = File::open(path).expect("File open error from path {path}");
    let buffered = BufReader::new(input);
    let result = buffered.lines().fold((0, 0), biggest_elf);

    println!("Result: {}",result.0);
    return Ok(());
}


fn biggest_elf(acc: (u32, u32), value: Result<String, std::io::Error>) -> (u32, u32) {
    let (max, current) = acc;
    let val_as_int: u32 = match value
        .expect("Input was no string in biggest_elf!")
        .parse() {
        Ok(int) => int,
        Err(_) => return (cmp::max(max, current), 0),
    };
    return (max, current + val_as_int);
}
