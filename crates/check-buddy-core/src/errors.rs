#![allow(unused)]

use thiserror::Error;
use crate::Piece;
use crate::position_move::Position;

#[derive(Error, Debug)]
pub enum PieceError {
    #[error("Can't find piece type from symbol")]
    SymbolNotFound,
}

#[derive(Error, Debug)]
pub enum PieceMoveError {
    #[error("You're trying to do an illegal move")]
    IllegalMove,
    #[error("Piece you're taking is not valid")]
    InvalidTakePiece,
    #[error("{0:?} on {1:?} is not yours")]
    NotYourPiece(Piece, Position),
}

#[derive(Error, Debug)]
pub enum HistoricalMoveError {
    #[error("Can't parse uci move")]
    InvalidUciMove,
    #[error("Can't parse uci move type")]
    InvalidUciMoveType,
    #[error("Couldn't find [from] position")]
    FromNotFound,
    #[error("Couldn't find [to] position")]
    ToNotFound,
}
