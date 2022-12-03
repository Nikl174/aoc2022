use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

/* CHEAT SHEET
 * Part 1:
 * A, X - Rock
 * B, Y - Paper
 * C, Z - Scissors
 *
 * A, Y - Win
 * B, Z - Win
 * C, X - Win
 * ...
 *   XYZ
 * A 360
 * B 036
 * C 603
 *
 * Part 2:
 * X - Loose
 * Y - Draw
 * Z - Win
 *   XYZ
 * A 312
 * B 123
 * C 231
 */

const VALUE_MATRIX_P1: [[usize; 3]; 3] = [[3, 6, 0], [0, 3, 6], [6, 0, 3]];
const POINT_MATRIX: [usize; 3] = [0, 3, 6];
const VALUE_MATRIX_P2: [[usize; 3]; 3] = [[3, 1, 2], [1, 2, 3], [2, 3, 1]];
const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;

fn main() {
    let file = get_puzzle_input().expect("Couldn't read file!");
    let result_part1 = file.lines().fold(0, calc_points_part1);
    let file = get_puzzle_input().expect("Couldn't read file!");
    let result_part2 = file.lines().fold(0, calc_points_part2);
    println!(
        "TotalPoints Part 1: {}; TotalPoints Part 2: {}",
        result_part1, result_part2
    );
}

// -------Utillity functions--------
fn convert(literal: char) -> Result<usize, std::io::Error> {
    match literal {
        'A' => return Ok(A),
        'B' => return Ok(B),
        'C' => return Ok(C),
        'X' => return Ok(X),
        'Y' => return Ok(Y),
        'Z' => return Ok(Z),
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Conversion Error",
            ))
        }
    }
}

fn get_puzzle_input() -> Result<BufReader<File>, std::io::Error> {
    let vars: Vec<String> = env::args().collect();
    let path = &vars[1];
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    return Ok(buffered);
}
// -------Utillity functions--------

// -------Fold function-------
fn calc_points_part1(point_sum: usize, new_points: Result<String, std::io::Error>) -> usize {
    let round = new_points.expect("Line read error");
    let mut round_iterator = round.chars();

    let enemy = round_iterator.next().expect("Input Error");
    round_iterator.next();
    let player = round_iterator.next().expect("Input Error");

    let enemy_index = convert(enemy).expect("Conversion Error!");
    let player_index = convert(player).expect("Conversion Error!");

    let round_sum = player_index + 1 + VALUE_MATRIX_P1[enemy_index][player_index];

    return round_sum + point_sum;
}

fn calc_points_part2(point_sum: usize, new_points: Result<String, std::io::Error>) -> usize {
    let round = new_points.expect("Line read error");
    let mut round_iterator = round.chars();

    let enemy = round_iterator.next().expect("Input Error");
    round_iterator.next();
    let player = round_iterator.next().expect("Input Error");

    let enemy_index = convert(enemy).expect("Conversion Error!");
    let player_index = convert(player).expect("Conversion Error!");

    let round_sum = VALUE_MATRIX_P2[enemy_index][player_index] + POINT_MATRIX[player_index];
    println!("Enemy: {enemy}; Player: {player}; Roundsum: {round_sum}");

    return round_sum + point_sum;
}

