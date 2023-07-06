#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PieceColor {
    Black,
    White,
}

impl PieceColor {
    pub fn to_value(self) -> u32 {
        match self {
            PieceColor::Black => 16,
            PieceColor::White => 8,
        }
    }
}
