#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum BoardState {
    #[default]
    Default,
    ShouldUpdate,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum PointerState {
    #[default]
    Default,
    PieceSelected,
}
