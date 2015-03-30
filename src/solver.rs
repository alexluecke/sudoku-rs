use puzzle::Puzzle;

#[derive(Debug)]
pub struct Solver {
    variable: i32,
}

impl Solver {
    pub fn solve(p: Puzzle) -> Vec<Puzzle> {
        let mut guesses: Vec<Puzzle> = Vec::new();

        guesses.push(p);

        guesses
    }
}
