#[derive(Debug)]
pub enum Source {
    Original,
    Add,
}

#[derive(Debug)]
pub struct Piece {
    pub source: Source,
    pub start: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct PieceTable {
    pub original: String,
    pub add: String,
    pub pieces: Vec<Piece>,
}
