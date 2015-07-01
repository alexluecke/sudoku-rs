use std::io;
use std::io::prelude::*;

extern crate SudokuSolver;

pub type Grid = Vec<Vec<u8>>;

pub mod helper;
pub mod sudoku;

use sudoku::Sudoku;

fn main() {

    let mut puzzle: Grid = Vec::new();
    let reader = io::stdin();
    for l in reader.lock().lines().take(9) {

        // need this or else lifetime is not long enough
        let line = l.unwrap().replace(" ", "");

        let rv: Vec<_> = line // row values
            .trim()
            .split(',')
            .take(9)
            .map(|x| {
                x.parse::<u8>().ok().expect("Expected a number.")
            })
            .collect();


        puzzle.push(rv);

    }

    let mut solver = Sudoku::new(puzzle);
    solver.solve();

}
