pub type Grid = Vec<Vec<u8>>;
pub type Coord = (usize, usize);

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

    pub fn solve(&mut self) {
        let mut empty: Vec<Coord> = Vec::new();
        let mut not_empty: Vec<Coord> = Vec::new();
        let mut simplification_found = true;

        while (simplification_found) {
            for row in 0..9 {
                for col in 0..9 {
                    self.remove_impossible((row, col));
                }
            }
            simplification_found = self.find_simplification();
        }

        for row in 0..9 {
            for col in 0..9 {
                println!("{:?}", self.possible[row][col]);
            }
        }

        for row in self.grid.iter() {
            println!("{:?}", row);
        }
    }

    fn find_simplification(&mut self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                // one item left means it must be the cell value
                if self.possible[row][col].len() == 1 {
                    self.grid[row][col] = self.possible[row][col].pop().unwrap();
                    return true;
                }
            }
        }
        return false;
    }

    fn remove_impossible(&mut self, coord: Coord) {
        let (n, m) = coord;
        let value = self.grid[n][m];
        for row in 0..9 {
            self.possible[row][m].retain(|&x| x != value);
        }
        for col in 0..9 {
            self.possible[n][col].retain(|&x| x != value);
        }
    }
}
