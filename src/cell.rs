
#[derive(Debug)]
pub struct Cell {
    pub data: Vec<i64>
}

impl Cell {
    pub fn new(x: i64) -> Cell {
        Cell { data:
            match x {
                -1 => range(1, 10).collect(),
                _ => range(1, 10).filter(|&i| x == i ).collect()
            }
        }
    }
}
