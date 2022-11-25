pub mod piece_move;
pub mod piece_type;

use piece_type::*;

#[derive(Copy, Clone)]
pub struct Piece(pub(crate) u32);

impl Piece {
    pub fn get_color(&self) -> PieceColor {
        if self.is_black() {
            PieceColor::Black
        } else {
            PieceColor::White
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
        // self.0 != 0 && self.0 != WHITE && self.0 != BLACK
        self.get_type().is_some() && self.0 != WHITE && self.0 != BLACK
    }

    pub fn get_icon(&self) -> Option<&str> {
        self.get_type()
            .map(|piece_type| match (piece_type, self.get_color()) {
                (PieceType::Rook, PieceColor::White) => "sprites/white_rook.png",
                (PieceType::Pawn(_), PieceColor::White) => "sprites/white_pawn.png",
                (PieceType::Bishop, PieceColor::White) => "sprites/white_bishop.png",
                (PieceType::Queen, PieceColor::White) => "sprites/white_queen.png",
                (PieceType::King, PieceColor::White) => "sprites/white_king.png",
                (PieceType::Knight, PieceColor::White) => "sprites/white_knight.png",
                (PieceType::Rook, PieceColor::Black) => "sprites/black_rook.png",
                (PieceType::Pawn(_), PieceColor::Black) => "sprites/black_pawn.png",
                (PieceType::Bishop, PieceColor::Black) => "sprites/black_bishop.png",
                (PieceType::Queen, PieceColor::Black) => "sprites/black_queen.png",
                (PieceType::King, PieceColor::Black) => "sprites/black_king.png",
                (PieceType::Knight, PieceColor::Black) => "sprites/black_knight.png",
            })
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PieceColor {
    Black,
    White,
}
