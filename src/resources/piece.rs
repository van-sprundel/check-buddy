use crate::resources::piece_type::*;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone)]
pub struct Piece(pub(crate) u32);

impl Piece {
    pub fn get_color(&self) -> bool {
        self.is_black()
    }
    pub fn get_type(&self) -> Option<PieceType> {
        let result = match self.0 % 8 {
            PAWN => PieceType::Pawn,
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
        (8..16).contains(&self.0)
    }
    pub(crate) fn is_black(&self) -> bool {
        (16..24).contains(&self.0)
    }
    pub(crate) fn is_piece(&self) -> bool {
        self.0 != 0 && self.0 != WHITE && self.0 != BLACK
    }

    pub(crate) fn get_icon(&self, asset_server: &Res<AssetServer>) -> Option<Handle<Image>> {
        self.get_type().map(|pt| {
            asset_server.load(match (pt, self.get_color()) {
                (PieceType::Rook, false) => "sprites/white_rook.png",
                (PieceType::Pawn, false) => "sprites/white_pawn.png",
                (PieceType::Bishop, false) => "sprites/white_bishop.png",
                (PieceType::Queen, false) => "sprites/white_queen.png",
                (PieceType::King, false) => "sprites/white_king.png",
                (PieceType::Knight, false) => "sprites/white_knight.png",
                (PieceType::Rook, true) => "sprites/black_rook.png",
                (PieceType::Pawn, true) => "sprites/black_pawn.png",
                (PieceType::Bishop, true) => "sprites/black_bishop.png",
                (PieceType::Queen, true) => "sprites/black_queen.png",
                (PieceType::King, true) => "sprites/black_king.png",
                (PieceType::Knight, true) => "sprites/black_knight.png",
            })
        })
    }
}

pub type Position = [usize; 2];

pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

pub const DIRECTION_OFFSETS: [i32; 8] = [8, 1, -8, -1, 9, -7, -9, 7];

impl Direction {
    pub fn from(index: usize) -> Self {
        match index {
            0 => Direction::North,     // 8
            1 => Direction::East,      // 1
            2 => Direction::South,     // -8
            3 => Direction::West,      //-1
            4 => Direction::NorthEast, // 9
            5 => Direction::SouthEast, // -7
            6 => Direction::SouthWest, // -9
            7 => Direction::NorthWest, //7
            _ => unreachable!(),
        }
    }
}
