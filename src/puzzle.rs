use std::fmt;
use self::CellStatus::{ SOLVED, UNSOLVED };
use helper::u8_to_ascii;

#[derive(PartialEq)]
pub enum CellStatus { SOLVED, UNSOLVED }

pub struct Puzzle {
    grid: Vec<Vec<Cell>>,
}

pub struct Cell {
    data: u8,
    status: CellStatus,
    possible: Vec<u8>
}

impl Cell {
    pub fn new(d: u8) -> Cell {

        let s = match d > 0 { true => SOLVED, _ => UNSOLVED };

        // Fill with possible numbers to guess and cull per cell
        let p = match d > 0 {
            true => vec![1,2,3,4,5,6,7,8,9],
            _ => vec![]
        };

        Cell { data: d, status: s, possible: p  }
    }

    pub fn decide(&mut self, v: u8) {
        self.data = v;
        self.status = SOLVED;
    }

    pub fn get_status(&self) -> &CellStatus {
        &self.status
    }

    pub fn get_data(&self) -> u8 {
        self.data
    }

}

impl Puzzle {
    pub fn new(v: Vec<Vec<Cell>>) -> Puzzle {
        Puzzle { grid: v }
    }

    pub fn get_grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }
}

impl fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.grid.iter() {
            println!("{:?}", row);
        }
        write!(f, "")
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
