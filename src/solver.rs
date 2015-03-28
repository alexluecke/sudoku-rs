use cell::Cell;
use puzzle::Puzzle;

#[derive(Debug)]
pub struct Solver {
    variable: i32,
}

impl Solver {
    pub fn solve(p: Puzzle) -> Vec<Puzzle> {
        let mut guesses: Vec<Puzzle> = Vec::new();
        let mut solutions: Vec<Puzzle> = Vec::new();

        guesses.push(p);

        //while !guesses.empty()) {
        //    guess = guesses.pop()
        //
        //    if guess.is_solved();
        //        solutions.push(guess);
        //}

        guesses
    }
}
