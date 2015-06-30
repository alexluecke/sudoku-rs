pub type Grid = Vec<Vec<u8>>;

pub struct Sudoku {
    grid: Grid,
    possible: Vec<Grid>
}

impl Sudoku {
    pub fn new(g: Grid) -> Sudoku {
        // Fill with possible numbers to guess and cull per cell
        let p = g.iter().map(|x| {
                x.iter().map(|&y|
                    match y > 0 {
                        true => vec![],
                        _ => vec![1,2,3,4,5,6,7,8,9]
                    }).collect()
            }).collect();

        Sudoku { grid: g, possible: p }
    }

    pub fn solve(&self) {
        let mut empty: Vec<(u8, u8)> = Vec::new();
        for row in 0..9 {
            for col in 0..9{
                if self.grid[row][col] == 0 {
                    assert_eq!(self.possible[row][col], [1,2,3,4,5,6,7,8,9]);
                    empty.push((row as u8, col as u8));
                }
            }
        }
    }
}
