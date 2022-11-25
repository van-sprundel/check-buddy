use check_buddy_core::piece::piece_move::Position;

pub struct PieceClickedEvent(pub Position);
pub struct PieceReleasedEvent(pub Position);
