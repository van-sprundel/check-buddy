#![allow(unused)]

use crate::position_move::Position;
use crate::Piece;
use thiserror::Error;

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
    #[error("You're trying to move a piece that's empty")]
    EmptySquare,
    #[error("{0:?} on {1:?} is not yours")]
    NotYourPiece(Piece, Position),
    #[error("Move not found")]
    MoveNotFound,
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
