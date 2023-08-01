mod digits_helper;
mod solver;

use crate::solver::*;
use regex::Regex;
use std::io;
use std::process;

fn main() {
    let mut input_string = String::new();
    println!("Enter expression in the format x op y = z where 0 < x, y, z < 1000 and op is either +, -, * or /");
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let re = Regex::new(r"(\d+) ([+\-*/]) (\d+) = (\d+)").unwrap();
    if !re.is_match(&input_string) {
        println!("Your input is not valid!");
        process::exit(1);
    }

    let start_expression = input_string.clone();

    println!("Enter 1 if you must move sticks, 2 if you must add and 3 if you must remove");
    input_string.clear();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let problem_type = input_string.trim().to_string().parse().expect("Input is not a valid integer");

    println!("Enter number of operations of previous type");
    input_string.clear();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let moves = input_string.trim().to_string().parse().expect("Input is not a valid integer");

    solve(start_expression, moves, match problem_type {
        1 => MOVE_TYPE,
        2 => ADD_TYPE,
        3 => REMOVE_TYPE,
        _ => ADD_TYPE
    });
}
