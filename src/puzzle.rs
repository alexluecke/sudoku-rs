#[derive(Debug)]
pub struct Puzzle {
    pub data: Vec<Cell>,
    pub dimension: u64,
}

impl Puzzle {
    pub fn new(arg_data: Vec<Cell>, arg_dim: u64) -> Puzzle {
        Puzzle { data: arg_data, dimension: arg_dim }
    }

    //pub fn is_solved(&self) -> bool {
        //for cell in self.data.iter() {
            //if *cell == -1 { return false }
        //}
        //return true
    //}
}
