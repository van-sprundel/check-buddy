use thiserror::Error;

#[derive(Debug, Error)]
pub enum PieceMoveError {
    #[error("You're trying to do an illegal move")]
    IllegalMove,
}
