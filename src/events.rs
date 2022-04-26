use crate::resources::piece::Position;

pub struct PieceClickedEvent(pub Position);
pub struct PieceReleasedEvent(pub Position);
