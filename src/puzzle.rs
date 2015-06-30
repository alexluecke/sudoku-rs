use std::fmt;
use self::CellStatus::{ SOLVED, UNSOLVED };
use helper::u8_to_ascii;

enum CellStatus { SOLVED, UNSOLVED }

pub struct Puzzle {
    cells: Vec<Cell>,
}

pub struct Cell {
    data: u8,
    status: CellStatus,
}

impl Cell {
    pub fn new(d: u8) -> Cell {
        let status = match d > 0 { true => SOLVED, _ => UNSOLVED };
        Cell { data: d, status: status }
    }

    pub fn decide(&mut self, v: u8) {
        self.data = v;
        self.status = SOLVED;
    }

    pub fn get_data(&self) -> u8 {
        self.data
    }
}

impl Puzzle {
    pub fn new(v: Vec<Cell>) -> Puzzle {
        Puzzle { cells: v }
    }

    pub fn from_vec(v: &[u8]) -> Puzzle {
        let data: Vec<_> = v.iter()
            .map(|&x| if x > 0 && x < 10 {
                Cell::new(x)
            } else {
                Cell::new(0)
            }).collect();
        Puzzle::new(data)
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }
}

impl fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut x = 1;
        for c in self.cells.iter() {
            s.push(u8_to_ascii(c.data));
            x += 1;
        }
        write!(f, "{}", s)
    }
}
