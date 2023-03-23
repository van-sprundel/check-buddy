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

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let piece = if self.is_black() {
            match (self.0 % 32) - BLACK {
                PAWN => "BP",
                KING => "BK",
                QUEEN => "BQ",
                ROOK => "BR",
                BISHOP => "BB",
                KNIGHT => "BN",
                _ => "□",
            }
        } else if self.is_white() {
            match (self.0 % 32) - WHITE {
                PAWN => "WP",
                KING => "WK",
                QUEEN => "WQ",
                ROOK => "WR",
                BISHOP => "WB",
                KNIGHT => "WN",
                _ => "■",
            }
        } else if self.0 == 100 {
            "▪"
        } else {
            ""
        };

        write!(f, "{:^2}", piece)
    }
}
