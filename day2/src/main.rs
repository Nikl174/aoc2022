use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

/* CHEAT SHEET
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
 */

const VALUE_MATRIX: [[usize; 3]; 3] = [[3, 6, 0], [0, 3, 6], [6, 0, 3]];
const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;

fn main() {
    let file = get_puzzle_input().expect("Couldn't read file!");
    let result = file.lines().fold(0, calc_points);
    println!("TotalPoints: {}",result);
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
fn calc_points(point_sum: usize, new_points: Result<String, std::io::Error>) -> usize {
    let round = new_points.expect("Line read error");
    let mut round_iterator = round.chars();

    let enemy = round_iterator.next().expect("Input Error");
    round_iterator.next();
    let player = round_iterator.next().expect("Input Error");

    let enemy_index = convert(enemy).expect("Conversion Error!");
    let player_index = convert(player).expect("Conversion Error!");
    println!("Player: {player}; Enemy: {enemy}; MatrixVal: {}",VALUE_MATRIX[enemy_index][player_index]);

    let round_sum =  player_index + 1 + VALUE_MATRIX[enemy_index][player_index];
    println!("Roundsum: {round_sum}; Current Sum: {}",round_sum+point_sum);

    return round_sum + point_sum;
}
