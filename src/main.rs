use std::io;
use std::io::prelude::*;

extern crate SudokuSolver;

pub mod puzzle;
pub mod helper;
pub mod sudoku;

use puzzle::Puzzle;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).ok()
        .expect("Please provide input.");

    let cells: Vec<_> = input.chars().take(81)
        .filter(|&c| c.to_digit(10) != None )
        .map(|x| x.to_digit(10).unwrap() as u8 )
        .collect();

    let p = Puzzle::from_vec(&cells);

    println!("{:?}", p);

}
