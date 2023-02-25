pub mod piece_move;
pub mod piece_type;

use piece_type::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Piece(pub u32);

impl Piece {
    pub fn get_color(&self) -> Option<PieceColor> {
        if self.is_black() {
            Some(PieceColor::Black)
        } else if self.is_white() {
            Some(PieceColor::White)
        } else {
            None
        }
    }
    pub fn get_type(&self) -> Option<PieceType> {
        let result = match self.0 % 8 {
            PAWN => PieceType::Pawn(self.0 > 32),
            KNIGHT => PieceType::Knight,
            KING => PieceType::King,
            ROOK => PieceType::Rook,
            QUEEN => PieceType::Queen,
            BISHOP => PieceType::Bishop,
            _ => return None,
        };
        Some(result)
    }
    pub(crate) fn is_white(&self) -> bool {
        (8..16).contains(&(self.0 % 32))
    }
    pub(crate) fn is_black(&self) -> bool {
        (16..24).contains(&(self.0 % 32))
    }
    pub fn is_piece(&self) -> bool {
        self.get_type().is_some() && self.0 != WHITE && self.0 != BLACK
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PieceColor {
    Black,
    White,
}
