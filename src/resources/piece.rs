use crate::resources::piece_type::*;
use bevy::asset::AssetServer;
use bevy::prelude::*;

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
    pub(crate) fn is_piece(&self) -> bool {
        // self.0 != 0 && self.0 != WHITE && self.0 != BLACK
        self.get_type().is_some() && self.0 != WHITE && self.0 != BLACK
    }

    pub(crate) fn get_icon(&self, asset_server: &Res<AssetServer>) -> Option<Handle<Image>> {
        self.get_type().map(|pt| {
            asset_server.load(match (pt, self.get_color()) {
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
        })
    }
}


#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Clone, Copy,Debug)]
pub struct PieceMove {
    pub from: Position,
    pub to: Position,
    pub en_passant: bool,
    pub trade:bool
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
pub const KNIGHT_DIRECTION_OFFSETS: [[i32; 2]; 8] = [
    //[Y,X]
    [1, -2],
    [2, -1],
    [2, 1],
    [1, 2],
    [-1, 2],
    [-2, 1],
    [-2, -1],
    [-1, -2],
];

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
