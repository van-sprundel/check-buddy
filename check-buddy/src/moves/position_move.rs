#[derive(Default, Clone, Copy, Debug)]
pub struct PositionMove {
    pub from: Position,
    pub to: Position,
    pub en_passant: bool,
    pub promotion: bool,
}

impl PositionMove {
    pub fn new(from: Position, to: Position) -> Self {
        Self {
            from,
            to,
            ..Default::default()
        }
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
