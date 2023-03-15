use crate::position_move::PositionMove;

pub type HistoricalMove = (UciMoveType, PositionMove);

#[derive(Debug, PartialEq)]
pub enum UciMoveType {
    Pawn { take: bool, check: bool, promotion: bool },
    CastleShort { take: bool, check: bool },
    CastleLong { take: bool, check: bool },
    Default { take: bool, check: bool },
    PawnPromote { take: bool, check: bool },
}

pub const NON_PAWN_SYMBOLS: [char; 5] = ['K', 'N', 'Q', 'R', 'B'];
