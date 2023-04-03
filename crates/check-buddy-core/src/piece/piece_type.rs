use crate::piece::Piece;
use std::fmt::{Debug, Formatter};

pub const NONE: u32 = 0;
pub const KING: u32 = 1;
pub const PAWN: u32 = 2;
pub const KNIGHT: u32 = 3;
pub const BISHOP: u32 = 4;
pub const ROOK: u32 = 5;
pub const QUEEN: u32 = 6;

pub const WHITE: u32 = 8;
pub const BLACK: u32 = 16;

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    Rook,
    Pawn(bool),
    King,
    Queen,
    Bishop,
    Knight,
}

impl PieceType {
    pub(crate) fn to_value(self) -> u32 {
        match self {
            PieceType::Rook => ROOK,
            PieceType::Pawn(_) => PAWN,
            PieceType::King => KING,
            PieceType::Queen => QUEEN,
            PieceType::Bishop => BISHOP,
            PieceType::Knight => KNIGHT
        }
    }
}
