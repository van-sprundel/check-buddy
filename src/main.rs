use std::cmp::min;
use std::fmt::{Debug, Formatter, write};
use std::io;
use colored::*;

fn main() {
    // let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/3Q4 w KQkq - 0 1");

    loop {
        println!("{:?}", board);

        let mut buffer = String::new();
        let mut stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();

        if buffer.len() < 4 {
            println!("{}", "Invalid syntax".red());
            continue;
        }

        // TODO make function to test which of the 2 possibilites can actually make the legal move
        let sections = buffer.chars().collect::<Vec<_>>();
        // let piece_to_move = sections[0];
        // let capture = sections[1] == 'x';
        // let start = if capture { 2 } else { 1 };
        // let move_to = sections[start..start + 2];
        let from_pos = &sections[0..2];
        let from_file = match from_pos[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => {
                println!("{}", "Invalid syntax".red());
                continue;
            }
        };
        let from_rank = *&from_pos[1].to_digit(10).unwrap() as usize;
        let from = [from_rank - 1, from_file];

        let to_pos = &sections[2..4];
        let to_file = match to_pos[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => {
                println!("{}", "Invalid syntax".red());
                continue;
            }
        };
        let to_rank = *&to_pos[1].to_digit(10).unwrap() as usize;
        let to = [to_rank - 1, to_file];

        if !(0..=8).contains(&to_file) || !(0..=8).contains(&from_file) || !(0..=8).contains(&to_rank) || !(0..=8).contains(&from_rank){
            println!("Move out of bounds");
            continue;
        }

        board.move_turn(from, to);
    }
}

struct Board {
    squares: [[Piece; 8]; 8],
    active_color: bool, // white is false, black is true
}

const NONE: u32 = 0;
const KING: u32 = 1;
const PAWN: u32 = 2;
const KNIGHT: u32 = 3;
const BISHOP: u32 = 4;
const ROOK: u32 = 5;
const QUEEN: u32 = 6;

const WHITE: u32 = 8;
const BLACK: u32 = 16;

impl Board {
    pub fn new() -> Self {
        let mut squares = [[Piece(0); 8]; 8];
        for x in 0..8 {
            for y in 0..8 {
                squares[x][y] = Piece(if (x + y) % 2 != 0 { BLACK } else { WHITE });
            }
        }
        Self {
            squares,
            active_color: false,
        }
    }
    /// starting position: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    /// white is uppercase, black is lowercase
    pub fn from_fen(fen: &str) -> Self {
        let mut board = Self::new();
        let sections = fen.split_whitespace().collect::<Vec<_>>();
        let placement = sections[0].split('/').collect::<Vec<_>>();

        let mut index = 0;
        placement.iter().for_each(|x| {
            for mut x in x.chars() {
                let color = if x.is_uppercase() { WHITE } else { BLACK };

                if !x.is_numeric() {
                    x.make_ascii_lowercase();
                    let rank = match x {
                        'p' => PAWN,
                        'r' => ROOK,
                        'b' => BISHOP,
                        'q' => QUEEN,
                        'k' => KING,
                        'n' => KNIGHT,
                        _ => 0
                    };
                    board.squares[7 - (index / 8)][index % 8] = Piece(color | rank);
                    index += 1;
                } else {
                    index += x.to_digit(10).unwrap() as usize;
                }
            }
        });
        board.active_color = if let Some(c) = sections[1].chars().nth(0) {
            match c {
                'w' => false,
                'b' => true,
                _ => unreachable!("FEN incorrect")
            }
        } else {
            false
        };

        board
    }
    pub fn move_turn(&mut self, on: [usize; 2], to: [usize; 2]) {
        let (x, y) = (on[0], on[1]);
        let piece = self.squares[x][y];
        if !piece.is_piece() {
            println!("You're trying to move a piece that's empty");
            return;
        }
        if (piece.is_white() && !self.active_color) ||
            (piece.is_black() && self.active_color) {
            if self.is_valid(on, to) {
                println!("{:?}", self);
                self.set_piece([x, y], if (x + y) % 2 != 0 { BLACK } else { WHITE });
                let (x, y) = (to[0], to[1]);
                self.set_piece([x, y], piece.0);
                self.active_color = !self.active_color;
            } else {
                println!("Move was invalid");
            }
        } else {
            println!("Piece is not yours");
        }
    }
    pub fn is_valid(&mut self, from: Move, to: Move) -> bool {
        let piece_from = self.squares[from[0]][from[1]];
        let piece_to = self.squares[to[0]][to[1]];

        if !piece_to.is_piece() || piece_to.get_color() != self.active_color {
            if let Some(piece_type) = piece_from.get_type() {
                match piece_type {
                    PieceType::Bishop | PieceType::Rook | PieceType::Queen => {
                        let moves = self.gen_sliding(from, to, piece_type);
                        return moves.contains(&to);
                    }
                    PieceType::Pawn => {}
                    PieceType::King => {}
                    PieceType::Knight => {}
                }
            } else {
                return true;
            }
            false
        } else {
            false
        }
    }
    pub fn set_piece(&mut self, on: Move, value: u32) {
        self.squares[on[0]][on[1]] = Piece(value);
    }
    pub fn get_fen(&self) -> String {
        todo!()
    }
    pub fn gen_sliding(&mut self, from: Move, to: Move, piece_type: PieceType) -> Vec<Move> {
        let mut moves = vec![];
        let start = if piece_type == PieceType::Bishop { 4 } else { 0 };
        let end = if piece_type == PieceType::Rook { 4 } else { 8 };
        for direction in start..end {
            for n in 0..self.len_to_edge(from, Direction::from(direction)) {
                let index = from[0] * 8 + from[1];
                let target_index = (index as i32 + DIRECTION_OFFSETS[direction] * (n + 1) as i32).clamp(0, 64) as usize;
                let target_move = [target_index / 8, target_index % 8];
                let target_piece = self.squares[target_move[0]][target_move[1]];

                if target_piece.is_piece() && target_piece.get_color() == self.active_color {
                    // your own color is in the way
                    break;
                }
                moves.push(target_move);
                // self.squares[target_move[0]][target_move[1]] = Piece(100);

                if target_piece.is_piece() && target_piece.get_color() != self.active_color {
                    // Enemy piece and capturable
                    break;
                }
            }
        }
        moves
    }
    pub fn len_to_edge(&self, pos: Move, direction: Direction) -> usize {
        let (rank, file) = (pos[0], pos[1]);
        let north = 7 - rank;
        let south = rank;
        let west = file;
        let east = 7 - file;

        match direction {
            Direction::North => north,
            Direction::NorthEast => min(north, east),
            Direction::East => east,
            Direction::SouthEast => min(south, east),
            Direction::South => south,
            Direction::SouthWest => min(south, west),
            Direction::West => west,
            Direction::NorthWest => min(north, west)
        }
    }
}

pub const DIRECTION_OFFSETS: [i32; 8] = [8, 9, 1, -7, -8, -9, -1, 7];

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..8 {
            writeln!(f, "{} {:?}", 8 - x, self.squares[7 - x]);
        }
        writeln!(f, "    a  b   c   d   e   f   g   h");
        if self.active_color {
            writeln!(f, "black turn");
        } else {
            writeln!(f, "white turn");
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct Piece(u32);

impl Piece {
    pub fn get_color(&self) -> bool {
        self.is_black()
    }
    pub fn get_type(&self) -> Option<PieceType> {
        let shift = if self.get_color() { BLACK } else { WHITE };
        let result = match self.0 - shift {
            PAWN => PieceType::Pawn,
            KNIGHT => PieceType::Knight,
            KING => PieceType::King,
            ROOK => PieceType::Rook,
            QUEEN => PieceType::Queen,
            BISHOP => PieceType::Bishop,
            _ => return None
        };
        Some(result)
    }
    fn is_white(&self) -> bool {
        (8..16).contains(&self.0)
    }
    fn is_black(&self) -> bool {
        (16..24).contains(&self.0)
    }
    fn is_piece(&self) -> bool {
        self.0 != 0 && self.0 != WHITE && self.0 != BLACK
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut piece = if self.is_black() {
            match self.0 - BLACK {
                PAWN => "BP",
                KING => "BK",
                QUEEN => "BQ",
                ROOK => "BR",
                BISHOP => "BB",
                KNIGHT => "BN",
                _ => "□"
            }
        } else if self.is_white() {
            match self.0 - WHITE {
                PAWN => "WP",
                KING => "WK",
                QUEEN => "WQ",
                ROOK => "WR",
                BISHOP => "WB",
                KNIGHT => "WN",
                _ => "■"
            }
        } else if self.0 == 100 { "▪" } else { "" };

        write!(f, "{:^2}", piece);
        Ok(())
    }
}

#[derive(PartialOrd, PartialEq)]
pub enum PieceType {
    Rook,
    Pawn,
    King,
    Queen,
    Bishop,
    Knight,
}

type Move = [usize; 2];

pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn from(index: usize) -> Self {
        match index {
            0 => Direction::North,
            1 => Direction::NorthEast,
            2 => Direction::East,
            3 => Direction::SouthEast,
            4 => Direction::South,
            5 => Direction::SouthWest,
            6 => Direction::West,
            7 => Direction::NorthWest,
            _ => unreachable!()
        }
    }
}