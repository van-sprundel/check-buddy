use crate::{piece_color::PieceColor, piece_type::PieceType, position_move::PositionMove};

pub type UciMove = (UciMoveType, PositionMove);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UciMoveType {
    Pawn {
        take: bool,
        check: bool,
        promotion: Option<PieceType>,
    },
    CastleShort {
        piece_color: PieceColor,
        take: bool,
        check: bool,
    },
    CastleLong {
        piece_color: PieceColor,
        take: bool,
        check: bool,
    },
    Default {
        piece_type: PieceType,
        specified_rank: bool,
        specified_file: bool,
        take: bool,
        check: bool,
    },
}

pub const NON_PAWN_SYMBOLS: [char; 5] = ['K', 'N', 'Q', 'R', 'B'];
