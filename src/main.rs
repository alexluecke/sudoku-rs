extern crate SudokuSolver;

use SudokuSolver::puzzle::Puzzle;
use SudokuSolver::solver::Solver;

use std::string::String;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut input = String::with_capacity(81);

    for line in stdin.lock().lines() {
        input = line.unwrap();
        break;
    }

    let int_values: Vec<i64> = input.chars()
        .map(|c| c as i64 - 48 )
        .map(|x| if x >= 0 && x <= 9 { x } else { -1 } )
        .collect();

    let puzzle = Puzzle::new(int_values, 9);
    let solutions = Solver::solve(puzzle);

    println!("{:?}", solutions);

}
