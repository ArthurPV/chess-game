pub struct Argument {
    pub argc: usize,
    pub argv: Vec<String>,
}

impl Argument {
    pub fn new() -> Argument {
        Argument {
            argc: std::env::args().len(),
            argv: std::env::args().collect(),
        }
    }
}
