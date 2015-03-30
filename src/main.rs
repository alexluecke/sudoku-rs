extern crate SudokuSolver;

use SudokuSolver::puzzle::Puzzle;
use SudokuSolver::cell::Cell;

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

    let cells: Vec<Cell> = input.chars()
        .map(|c| c as i64 - 48 )
        .map(|x| if x >= 0 && x <= 9 {
                Cell::new(x)
            } else {
                Cell::new(-1)
            })
        .collect();

    //println!("{:?}", cells);

    let puzzle = Puzzle::new(cells, 9);

    println!("{:?}", puzzle);

    //let solutions = Solver::solve(puzzle);


}
