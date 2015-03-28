
#[derive(Debug)]
pub struct Cell {
    data: Vec<i64>
}

impl Cell {
    pub fn new() -> Cell {
        Cell { data: Vec::new() }
    }
}
