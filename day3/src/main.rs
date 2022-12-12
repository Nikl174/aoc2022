use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = get_puzzle_input().expect("Couldn't read from File").lines();
    let solution = lines.fold(0, line_fold);
    let lines = get_puzzle_input().expect("Couldn't read from File").lines();
    let solution2 = lines.fold(([(0, false); 52], 0), group_fold).1;

    println!("Sum1: {solution}; Sum2: {solution2}");
}

fn group_fold(
    acc: ([(usize, bool); 52], usize),
    input: Result<String, std::io::Error>,
) -> ([(usize, bool); 52], usize) {
    let (mut alphabet, mut current_result) = acc;
    let backpack = input.expect("No line");

    println!("Word: {backpack}");
    //print!("Alphabet before: ");
    for chr in backpack.chars().into_iter() {
        let i = convert_to_index(chr);
        //print!("{}", alphabet[i].0);
        if !alphabet[i].1 {
            if alphabet[i].0 + 1 < 3 {
                alphabet[i].1 = true;
                alphabet[i].0 = alphabet[i].0 + 1;
            } else {
                current_result += i +1;

                print!("Alphabet ret: ");
                for mut i in alphabet.iter_mut() {
                    i.1 = false;
                    i.0 = 0;
                    print!("{}", i.0);
                }
                println!("");
                println!("{current_result} inc: {i}");
                return (alphabet, current_result);
            }
        }
    }
    //println!("");

    print!("Alphabet ret: ");
    for mut i in alphabet.iter_mut() {
        i.1 = false;
        print!("{}", i.0);
    }
    println!("");
    println!("{current_result}");
    return (alphabet, current_result);
}

fn line_fold(acc: usize, input: Result<String, std::io::Error>) -> usize {
    let mut first_comp: [usize; 52] = [0; 52];
    let backpack = input.expect("No line");
    let str_split = backpack.split_at(backpack.len() / 2);

    for chr in str_split.0.to_string().chars().into_iter() {
        let i = convert_to_index(chr);
        first_comp[i] = first_comp[i] + 1;
    }

    for chr in str_split.1.to_string().chars().into_iter() {
        let i = convert_to_index(chr);
        if first_comp[i] >= 1 {
            return acc + i + 1;
        }
    }
    return acc;
}

fn convert_to_index(letter: char) -> usize {
    let ascii = letter as usize;
    if letter.is_ascii_lowercase() {
        return ascii - ('a' as usize);
    } else {
        return 26 + ascii - ('A' as usize);
    }
}

fn get_puzzle_input() -> Result<BufReader<File>, std::io::Error> {
    let vars: Vec<String> = env::args().collect();
    let path = &vars[1];
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    return Ok(buffered);
}
