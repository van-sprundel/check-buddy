use crate::errors::*;
use crate::historical_move::{HistoricalMove, UciMoveType, NON_PAWN_SYMBOLS};
use crate::piece::{piece_type::*, Piece, PieceColor};
use crate::position_move::{
    Direction, Position, PositionMove, DIRECTION_OFFSETS, KNIGHT_DIRECTION_OFFSETS,
};
use anyhow::{anyhow, Result};
use std::borrow::BorrowMut;
use std::cmp::min;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut, Sub};

const RANKS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

#[derive(Clone, Copy)]
pub struct BoardMap {
    squares: [[Piece; 8]; 8],
    active_color: PieceColor, // white is false, black is true
}

impl Default for BoardMap {
    fn default() -> Self {
        let squares = [[Piece(0); 8]; 8];

        Self {
            squares,
            active_color: PieceColor::White,
        }
    }
}

impl BoardMap {
    pub fn empty() -> Self {
        BoardMap::default()
    }
    /// starting position: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    pub fn starting() -> Self {
        BoardMap::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
    pub fn from_fen(fen: impl Into<String>) -> Self {
        let fen = fen.into();
        let mut board = Self::default();
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
                        _ => 0,
                    };
                    board.squares[(index / 8)][index % 8] = Piece(color | rank);
                    index += 1;
                } else {
                    index += x.to_digit(10).unwrap() as usize;
                }
            }
        });
        board.active_color = if let Some(string) = sections.get(1) {
            if let Some(c) = string.chars().next() {
                match c {
                    'w' => PieceColor::White,
                    'b' => PieceColor::Black,
                    _ => unreachable!("FEN incorrect"),
                }
            } else {
                PieceColor::White
            }
        } else {
            PieceColor::White
        };

        board
    }
    pub fn get_fen(&self) -> String {
        let mut fen = String::new();
        let mut squares = self.squares;
        squares.reverse();
        for row in squares {
            let mut space = 0;
            for col in row {
                if let Some(piece_type) = col.get_type() {
                    if space != 0 {
                        fen.push_str(space.to_string().as_str());
                        space = 0;
                    }

                    match piece_type {
                        PieceType::Rook => fen.push(if col.get_color() == PieceColor::Black {
                            'r'
                        } else {
                            'R'
                        }),
                        PieceType::Pawn(_) => fen.push(if col.get_color() == PieceColor::Black {
                            'p'
                        } else {
                            'P'
                        }),
                        PieceType::King => fen.push(if col.get_color() == PieceColor::Black {
                            'k'
                        } else {
                            'K'
                        }),
                        PieceType::Queen => fen.push(if col.get_color() == PieceColor::Black {
                            'q'
                        } else {
                            'Q'
                        }),
                        PieceType::Bishop => fen.push(if col.get_color() == PieceColor::Black {
                            'b'
                        } else {
                            'B'
                        }),
                        PieceType::Knight => fen.push(if col.get_color() == PieceColor::Black {
                            'n'
                        } else {
                            'N'
                        }),
                    }
                } else {
                    space += 1;
                }
            }
            if space != 0 {
                fen.push_str(space.to_string().as_str());
            }
            fen.push('/');
        }

        fen.pop();
        fen
    }
    pub fn parse_pgn_to_uci_move(&mut self, mut uci: &str) -> Result<HistoricalMove> {
        let check = uci.chars().next() == Some('+');
        if check {
            let mut chars = uci.chars();
            chars.next();
            uci = chars.as_str();
        }

        let uci_move_type = if uci.len() == 2 {
            UciMoveType::Pawn {
                take: false,
                check,
                promotion: false,
            }
        } else if uci.len() == 4 && RANKS.contains(&uci.chars().nth(0).unwrap()) {
            let take = uci.chars().nth(2) == Some('x');
            UciMoveType::Pawn {
                take,
                check,
                promotion: false,
            }
        } else if uci == "O-O" {
            UciMoveType::CastleShort { take: false, check }
        } else if uci == "O-O-O" {
            UciMoveType::CastleLong { take: false, check }
        } else if uci.len() >= 3 && NON_PAWN_SYMBOLS.contains(&uci.chars().nth(0).unwrap()) {
            let specified_rank = RANKS.contains(&uci.chars().nth(1).unwrap());
            let take = (!specified_rank && uci.chars().nth(1) == Some('x'))
                || (specified_rank && uci.chars().nth(2) == Some('x'));

            //TODO
            // Re8     - Rook moves to e8
            // Rxe8    - Rook takes e8
            // RxQ     - Rook takes Queen
            // Rfe8    - Rook on f rank moves to e8
            // Rfxe8   - Rook on f rank takes e8
            // RfxQ    - Rook on f rank takes Queen
            let piece_type = match uci.chars().nth(0).unwrap() {
                'K' => PieceType::King,
                'N' => PieceType::Knight,
                'Q' => PieceType::Queen,
                'R' => PieceType::Rook,
                'B' => PieceType::Bishop,
                _ => return Err(PieceError::SymbolNotFound.into()),
            };

            UciMoveType::Default {
                piece_type,
                take,
                check,
            }
        } else {
            return Err(HistoricalMoveError::InvalidUciMoveType.into());
        };

        let to: Position = match uci_move_type {
            UciMoveType::Pawn { take, .. } => {
                let to = if take {
                    uci.chars().skip(2).take(2).collect::<String>()
                } else {
                    uci.chars().take(2).collect::<String>()
                };
                self.parse_uci_position_to_file_rank(to)?
            }
            UciMoveType::PawnPromote { take, .. } => {
                let to = if take {
                    uci.chars().skip(2).take(2).collect::<String>()
                } else {
                    uci.chars().take(2).collect::<String>()
                };
                self.parse_uci_position_to_file_rank(to)?
            }
            UciMoveType::Default { take, .. } => {
                if take {
                    let to = uci.chars().skip(2).take(2).collect::<String>();
                    let pos = self.parse_uci_position_to_file_rank(to)?;
                    let piece = self.get_piece(pos);
                    if !piece.is_piece() || self.get_active_color() == &piece.get_color() {
                        return Err(anyhow!(PieceMoveError::InvalidTakePiece));
                    }
                    pos
                } else {
                    let to = uci.chars().skip(1).take(2).collect::<String>();
                    self.parse_uci_position_to_file_rank(to)?
                }
            }
            UciMoveType::CastleShort { .. } => {
                if self.get_active_color() == &PieceColor::White {
                    [7, 1]
                } else {
                    [0, 1]
                }
            }
            UciMoveType::CastleLong { .. } => {
                if self.get_active_color() == &PieceColor::White {
                    [7, 5]
                } else {
                    [0, 5]
                }
            }
        };

        let from: Position = match uci_move_type {
            UciMoveType::Pawn {
                take, promotion, ..
            } => {
                if take {
                    let shift = match self.get_active_color() {
                        PieceColor::Black => -1,
                        PieceColor::White => 1,
                    };
                    let pos1 = [((to[0] as i32) + shift) as usize, to[1]];
                    let pos2 = [
                        ((to[0] as i32) + shift) as usize,
                        ((to[1] as i32) + 1) as usize,
                    ];
                    let pos3 = [
                        ((to[0] as i32) + shift) as usize,
                        ((to[1] as i32) - 1) as usize,
                    ];

                    self.verify_any_own_position(vec![pos1, pos2, pos3])?
                } else if promotion {
                    let should_be_on_file = match self.get_active_color() {
                        PieceColor::Black => 6,
                        PieceColor::White => 1,
                    };

                    let mut possible_position = None;

                    for (rank, square) in self.squares[should_be_on_file].iter().enumerate() {
                        if square.is_piece()
                            && square.get_type().unwrap() == PieceType::Pawn(false)
                            && square.get_color() == *self.get_active_color()
                        {
                            possible_position = Some([rank, should_be_on_file]);
                            break;
                        }
                    }

                    if possible_position.is_some() {
                        possible_position.unwrap()
                    } else {
                        return Err(anyhow!(HistoricalMoveError::FromNotFound));
                    }
                } else {
                    let shift = match self.get_active_color() {
                        PieceColor::Black => -1,
                        PieceColor::White => 1,
                    };
                    let pos1 = [((to[0] as i32) + shift) as usize, to[1]];
                    let pos2 = [((to[0] as i32) + shift * 2) as usize, to[1]];
                    self.verify_any_own_position(vec![pos1, pos2])?
                }
            }
            UciMoveType::Default { piece_type, .. } => {
                let possible_positions = self.get_positions_from_type(&piece_type);
                let mut found_position = None;
                for position in possible_positions.iter() {
                    let moves = self.gen_to_positions(*position);
                    if moves.contains(&to) {
                        found_position = Some(*position);
                        break;
                    }
                }
                if let Some(position) = found_position {
                    position
                } else {
                    return Err(anyhow!(
                        "Couldn't find [from] position on piece with [to] {:?}",
                        to
                    ));
                }
            }
            UciMoveType::CastleShort { .. } | UciMoveType::CastleLong { .. } => {
                if self.get_active_color() == &PieceColor::White {
                    [7, 3]
                } else {
                    [0, 3]
                }
            }
            UciMoveType::PawnPromote { .. } => {
                unimplemented!()
            }
        };

        Ok((uci_move_type, PositionMove::new(from, to)))
    }
    pub fn get_piece(&self, pos: Position) -> Piece {
        self.squares[pos[0]][pos[1]]
    }
    pub fn find_piece(&self, piece_color: PieceColor, piece_type: PieceType) -> Vec<Position> {
        let mut vec = vec![];
        for (y, row) in self.squares.iter().enumerate() {
            for (x, p) in row.iter().enumerate() {
                if let Some(t) = p.get_type() {
                    if t == piece_type && p.get_color() == piece_color {
                        vec.push([y, x]);
                    }
                }
            }
        }
        vec
    }
    pub fn get_piece_mut(&mut self, pos: Position) -> &mut Piece {
        self.squares[pos[0]][pos[1]].borrow_mut()
    }
    pub fn get_active_color(&self) -> &PieceColor {
        &self.active_color
    }
    pub fn get_active_pieces(&self) -> Vec<Position> {
        let mut pieces = vec![];
        for (i, row) in self.squares.iter().enumerate() {
            for (j, piece) in row.iter().enumerate() {
                if piece.get_color() == *self.get_active_color() {
                    pieces.push([i, j]);
                }
            }
        }
        pieces
    }
    pub fn set_piece(&mut self, on: Position, value: u32) {
        self.squares[on[0]][on[1]] = Piece(value);
    }
    /// makes a move with uci info
    ///
    /// returns true if move was successful
    pub fn uci_move_turn(&mut self, uci_move: HistoricalMove) -> Result<()> {
        let temp_board = *self;

        let position_move = uci_move.1;

        let PositionMove { from, to, .. } = position_move;
        let (x, y) = (from[0], from[1]);
        let piece = self.squares[x][y];

        let should_enable_en_passant = self.move_should_enable_en_passant(position_move);
        self.is_valid_move(position_move)?;

        self.make_move(position_move);

        let piece_to = &mut self.get_piece(to);

        // TODO for en passant could be removed
        if let Some(PieceType::Pawn(_)) = piece_to.get_type() {
            if should_enable_en_passant {
                // eprintln!("Piece became en passantable! ({},{})", to[0], to[1]);
                self.get_piece_mut(to).0 += 32;
            } else if self.get_piece(to).0 > 32 {
                self.get_piece_mut(to).0 -= 32;
            }
        }

        *self = temp_board;
        self.switch_active_color();
        Ok(())
    }
    /// makes a single move with check
    ///
    /// returns true if move was successful
    pub fn single_move_turn(&mut self, position_move: PositionMove) -> Result<()> {
        let temp_board = *self;

        let PositionMove { from, to, .. } = position_move;
        let (x, y) = (from[0], from[1]);
        let piece = self.squares[x][y];

        let should_enable_en_passant = self.move_should_enable_en_passant(position_move);
        self.is_valid_move(position_move)?;

        self.make_move(position_move);

        let piece_to = &mut self.get_piece(to);

        // TODO for en passant could be removed
        if let Some(PieceType::Pawn(_)) = piece_to.get_type() {
            if should_enable_en_passant {
                // eprintln!("Piece became en passantable! ({},{})", to[0], to[1]);
                self.get_piece_mut(to).0 += 32;
            } else if self.get_piece(to).0 > 32 {
                self.get_piece_mut(to).0 -= 32;
            }
        }

        *self = temp_board;
        self.switch_active_color();

        Ok(())
    }
    /// check if move is valid
    pub fn is_valid_move(&self, piece_move: PositionMove) -> Result<()> {
        let PositionMove { from, to, .. } = piece_move;
        let piece_from = self.squares[from[0]][from[1]];
        let piece_to = self.squares[to[0]][to[1]];

        if !piece_from.is_piece() {
            return Err(anyhow!(PieceMoveError::EmptySquare));
        }

        if piece_from.get_color() != self.active_color {
            return Err(anyhow!(PieceMoveError::NotYourPiece(piece_from, from)));
        }

        let moves = self.gen_legal_positions(from);

        if !moves.contains(&to) {
            return Err(anyhow!(PieceMoveError::MoveNotFound));
        }

        Ok(())
    }
    pub fn is_hit(&self, pos: Position) -> bool {
        let piece_on = self.get_piece(pos);
        piece_on.is_piece() && piece_on.get_color() != self.active_color
    }
    /// generate only legal move positions for piece
    pub fn gen_legal_positions(&self, from: Position) -> Vec<Position> {
        let mut temp_board = *self;
        let moves = temp_board.gen_to_positions(from);
        let mut legal_moves = vec![];
        // eprintln!("moves {:?}", &moves);

        for to in moves.into_iter() {
            let position_move = PositionMove::new(from, to);
            let en_passant = self.is_en_passant(position_move);
            let promotion = self.is_promotion(position_move);
            let last_piece = temp_board.squares[to[0]][to[1]].0;

            let position_move = PositionMove {
                from,
                to,
                en_passant,
                promotion,
            };
            temp_board.make_move(position_move);
            let next_moves = temp_board.gen_all_opponent_positions();
            // eprintln!("next possible moves: {:?}", next_moves);
            if !next_moves.iter().any(|x| {
                let next_piece = temp_board.squares[x[0]][x[1]];
                if next_piece.is_piece() && next_piece.get_color() == temp_board.active_color {
                    // eprintln!("{:?}", next_piece);
                    return Some(PieceType::King) == next_piece.get_type();
                }
                false
            }) {
                legal_moves.push(to);
            }

            temp_board.undo_move(position_move, last_piece);
        }
        // eprintln!("legal moves {:?}", &legal_moves);
        legal_moves
    }
    /// generate all possible moves for piece
    pub fn gen_to_positions(&self, from: Position) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        if let Some(piece_type) = piece_from.get_type() {
            return match piece_type {
                PieceType::Bishop | PieceType::Rook | PieceType::Queen => {
                    self.gen_sliding(from, piece_type)
                }
                PieceType::Pawn(_) => self.gen_pawn(from),
                PieceType::King => self.gen_king(from),
                PieceType::Knight => self.gen_knight(from),
            };
        }
        vec![]
    }
    pub fn gen_sliding(&self, from: Position, piece_type: PieceType) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        let mut moves = vec![];
        let start = if piece_type == PieceType::Bishop {
            4
        } else {
            0
        };
        let end = if piece_type == PieceType::Rook { 4 } else { 8 };
        for (direction, offset) in DIRECTION_OFFSETS.iter().enumerate().take(end).skip(start) {
            for n in 0..self.len_to_edge(from, Direction::from(direction)) {
                let index = from[0] * 8 + from[1];
                let target_index = (index as i32 + offset * (n + 1) as i32).clamp(0, 63) as usize;
                let target_move = [target_index / 8, target_index % 8];
                let target_piece = self.squares[target_move[0]][target_move[1]];

                if target_piece.is_piece() && target_piece.get_color() == piece_from.get_color() {
                    // your own color is in the way
                    // eprintln!("Piece is yours! {:?}",target_move);
                    break;
                }
                moves.push(target_move);
                // self.squares[target_move[0]][target_move[1]] = Piece(100);

                if target_piece.is_piece() && target_piece.get_color() != piece_from.get_color() {
                    // Enemy piece and capturable
                    // eprintln!("Piece is not yours, but you should still break the loop!");
                    break;
                }
            }
        }
        moves
    }
    pub fn gen_king(&self, from: Position) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        let mut moves = vec![];
        for (direction, offset) in DIRECTION_OFFSETS.iter().enumerate() {
            let index = from[0] * 8 + from[1];
            let target_index = index as i32 + offset;
            if !(0..=63).contains(&target_index)
                || self.len_to_edge(from, Direction::from(direction)) == 0
            {
                continue;
            }
            let target_move = [target_index as usize / 8, target_index as usize % 8];
            let target_piece = self.squares[target_move[0]][target_move[1]];

            if target_piece.is_piece() && target_piece.get_color() == piece_from.get_color() {
                // your own color is in the way
                // eprintlnln!("Piece is yours!");
                continue;
            }
            moves.push(target_move);
            // self.squares[target_move[0]][target_move[1]] = Piece(100);

            if target_piece.is_piece() && target_piece.get_color() != piece_from.get_color() {
                // Enemy piece and capturable
                // eprintln!("A piece that's yours is blocking any other moves");
                continue;
            }
        }
        moves
    }
    pub fn gen_pawn(&self, from: Position) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        let mut moves = vec![];
        let shift = match piece_from.get_color() {
            PieceColor::Black => 1,
            PieceColor::White => -1,
        };

        // piece blocking
        let vertical = (from[0] as i32 + shift) as usize;
        if vertical < 8 {
            let is_blocking = self.squares[vertical][from[1]].is_piece();
            if !is_blocking {
                moves.push([(from[0] as i32 + shift) as usize, from[1]]);
            }

            // hasn't moved yet
            let vertical = (from[0] as i32 + shift * 2) as usize;
            if vertical < 8 {
                let is_blocking = is_blocking || self.squares[vertical][from[1]].is_piece();
                if ((piece_from.is_black() && from[0] == 1)
                    || (piece_from.is_white() && from[0] == 6))
                    && !is_blocking
                {
                    moves.push([vertical, from[1]]);
                }
            }
        }

        // takeable pieces on [+1,-1]
        // x  .  .
        // .  p  .
        if from[1] > 0 && from[1] < 8 {
            let to_top_left_pos = [(from[0] as i32 + shift) as usize, from[1] - 1];
            if to_top_left_pos[0] < 8 {
                let to_top_left = self.get_piece(to_top_left_pos);
                if to_top_left.is_piece() && to_top_left.get_color() != piece_from.get_color() {
                    moves.push(to_top_left_pos);
                }

                // en passant
                // x  .  .
                // _  p  .
                let to_left = self.squares[from[0]][from[1] - 1];
                if let Some(PieceType::Pawn(en_passantable)) = to_left.get_type() {
                    if en_passantable && to_left.get_color() != piece_from.get_color() {
                        // eprintln!("piece on left ({:?}) is en passantable!", [from[0], from[1] - 1]);
                        let to_en_passant = [(from[0] as i32 + shift) as usize, from[1] - 1];
                        moves.push(to_en_passant);
                    }
                }
            }
        }

        // takeable pieces on [+1,+1]
        // .  .  x
        // .  p  .
        if from[1] < 7 {
            let to_top_right_pos = [(from[0] as i32 + shift) as usize, from[1] + 1];
            if to_top_right_pos[0] < 8 {
                let to_top_right = self.squares[to_top_right_pos[0]][to_top_right_pos[1]];
                if to_top_right.is_piece() && to_top_right.get_color() != piece_from.get_color() {
                    moves.push(to_top_right_pos);
                }

                // en passant
                // .  .  x
                // .  p  _
                let to_right = self.squares[from[0]][from[1] + 1];
                if let Some(PieceType::Pawn(en_passantable)) = to_right.get_type() {
                    if en_passantable && to_right.get_color() != piece_from.get_color() {
                        // eprintln!("piece on right ({:?}) is en passantable!", [from[0], from[1] + 1]);
                        let to_en_passant = [(from[0] as i32 + shift) as usize, from[1] + 1];
                        moves.push(to_en_passant);
                    }
                }
            }
        }
        moves
    }
    pub fn gen_knight(&self, from: Position) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        KNIGHT_DIRECTION_OFFSETS
            .iter()
            .filter_map(|direction| {
                let new_pos = [
                    (direction[0] + from[0] as i32) as usize,
                    (direction[1] + from[1] as i32) as usize,
                ];
                if new_pos[0] < 8 && new_pos[1] < 8 {
                    let target_piece = self.squares[new_pos[0]][new_pos[1]];
                    if !(target_piece.is_piece()
                        && target_piece.get_color() == piece_from.get_color())
                    {
                        return Some(new_pos);
                    }
                }
                None
            })
            .collect()
    }
    fn len_to_edge(&self, pos: Position, direction: Direction) -> usize {
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
            Direction::NorthWest => min(north, west),
        }
    }
    /// make a move (without check)
    pub fn make_move(&mut self, position_move: PositionMove) {
        let PositionMove {
            from,
            to,
            en_passant,
            promotion,
        } = position_move;

        if en_passant {
            // eprintln!("move is an en passant!");
            let shift = if self.get_piece(from).get_color() == PieceColor::Black {
                -1
            } else {
                1
            };
            let to_step = [(to[0] as isize + shift) as usize, to[1]];
            if to_step[0] != 0 && to_step[1] != 0 && shift != -1 && to_step[0] != 8 {
                self.set_piece(to_step, 0);
            }
        } else if promotion {
            let color = match self.get_piece(from).get_color() {
                PieceColor::Black => BLACK,
                PieceColor::White => WHITE,
            };
            self.set_piece(to, QUEEN | color);
        } else {
            self.set_piece(to, self.get_piece(from).0);
        }
        self.set_piece(from, 0);
    }
    pub fn undo_move(&mut self, piece_move: PositionMove, last_piece: u32) {
        let PositionMove { from, to, .. } = piece_move;
        self.set_piece(from, self.get_piece(to).0);
        self.set_piece(to, last_piece);
    }
    /// generates all moves based on active color.
    pub fn gen_all_legal_moves(&self) -> Vec<PositionMove> {
        let mut legal_moves = vec![];
        for rank in 0..8 {
            for file in 0..8 {
                let piece = self.squares[rank][file];
                if piece.is_piece() && piece.get_color() == self.active_color {
                    let from_move = [rank, file];
                    let to_moves = self.gen_legal_positions([rank, file]);
                    let moves = to_moves
                        .iter()
                        .map(|&to| PositionMove::new(from_move, to))
                        .collect::<Vec<_>>();
                    legal_moves.extend(moves);
                }
            }
        }
        legal_moves
    }
    pub fn gen_all_opponent_positions(&self) -> Vec<Position> {
        let mut opponent_positions = vec![];
        for rank in 0..8 {
            for file in 0..8 {
                let piece = self.squares[rank][file];
                if piece.is_piece() && piece.get_color() != self.active_color {
                    // eprintln!("found enemy piece! {:?}", piece);
                    let moves = self.gen_to_positions([rank, file]);
                    opponent_positions.extend(moves);
                }
            }
        }
        opponent_positions
    }
    pub fn is_en_passant(&self, piece_move: PositionMove) -> bool {
        // only en passant moves can be moved diagonally on an empty square
        let PositionMove { from, to, .. } = piece_move;
        let piece = self.get_piece(from);
        if let Some(piece_type) = piece.get_type() {
            if piece_type == PieceType::Pawn(false) || piece_type == PieceType::Pawn(true) {
                let shift = match piece.get_color() {
                    PieceColor::Black => 1,
                    PieceColor::White => -1,
                };
                let step_pos = [(to[0] as isize - shift) as usize, to[1]];
                let step_piece = self.get_piece(step_pos);
                if step_piece.is_piece() && step_piece.get_color() != piece.get_color() {
                    if let Some(step_piece_type) = step_piece.get_type() {
                        if step_piece_type == PieceType::Pawn(false)
                            || step_piece_type == PieceType::Pawn(true)
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    pub fn is_promotion(&self, piece_move: PositionMove) -> bool {
        let PositionMove { from, to, .. } = piece_move;
        let piece = self.get_piece(from);
        if let Some(piece_type) = piece.get_type() {
            return (piece_type == PieceType::Pawn(false) || piece_type == PieceType::Pawn(true))
                && (to[0] == 7 || to[0] == 0);
        }

        false
    }
    // f(p) = 200(K-K')
    //        + 9(Q-Q')
    //        + 5(R-R')
    //        + 3(B-B' + N-N')
    //        + 1(P-P')
    //        - 0.5(D-D' + S-S' + I-I')
    //        + 0.1(M-M') + ...
    // KQRBNP = number of kings, queens, rooks, bishops, knights and pawns
    //  D,S,I = doubled, blocked and isolated pawns
    //  M = Mobility (the number of legal moves)
    pub fn get_material_weight(&self) -> i32 {
        let mut res = 0;
        for row in self.squares.iter() {
            for piece in row.iter() {
                let mut value = piece.0;
                if value >= 32 {
                    value %= 32;
                } // en passantable pawns
                let piece_value =
                    if self.active_color == PieceColor::White && value > WHITE && value < BLACK {
                        value - WHITE
                    } else if self.active_color == PieceColor::Black && value > BLACK {
                        value - BLACK
                    } else {
                        continue;
                    };

                let mut piece_weight = match piece_value {
                    PAWN => 1,
                    KNIGHT | BISHOP => 3,
                    ROOK => 5,
                    QUEEN => 9,
                    KING => 200,
                    _ => unimplemented!("{} is not a valid piece value", piece.0),
                };
                if self.active_color != piece.get_color() {
                    piece_weight *= -1;
                }
                res += piece_weight;
            }
        }
        res
    }
    pub fn get_num_white_pieces(&self) -> i32 {
        self.squares.iter().fold(0, |res, row| {
            res + row
                .iter()
                .filter(|&&item| item.0 > WHITE && item.0 < BLACK)
                .count()
        }) as i32
    }
    pub fn get_num_black_pieces(&self) -> i32 {
        self.squares.iter().fold(0, |res, row| {
            res + row.iter().filter(|&&item| item.0 > BLACK).count()
        }) as i32
    }
    fn get_positions_from_type(&self, piece_type: &PieceType) -> Vec<Position> {
        let mut possible_positions = vec![];
        for (i, row) in self.squares.iter().enumerate() {
            for (j, position) in row.iter().enumerate() {
                if position.is_piece() {
                    if let Some(pt) = position.get_type() {
                        if piece_type == &pt {
                            possible_positions.push([i, j]);
                        }
                    }
                }
            }
        }
        possible_positions
    }
    fn switch_active_color(&mut self) {
        self.active_color = if self.active_color == PieceColor::Black {
            PieceColor::White
        } else {
            PieceColor::Black
        };
    }
    fn move_should_enable_en_passant(&self, piece_move: PositionMove) -> bool {
        let PositionMove { from, to, .. } = piece_move;
        let piece = self.get_piece(from);
        if let Some(PieceType::Pawn(_)) = piece.get_type() {
            if *self.get_active_color() == piece.get_color() {
                return match piece.get_color() {
                    PieceColor::White => from[0] == 6 && to[0] == 4,
                    PieceColor::Black => from[0] == 1 && to[0] == 3,
                };
            }
        }
        false
    }

    fn parse_uci_position_to_file_rank(&self, mut position: String) -> Result<Position> {
        let file = 7 - position
            .pop()
            .ok_or(anyhow!("can't parse"))?
            .to_string()
            .parse::<usize>()?
            .sub(1);

        let rank = (position.pop().ok_or(anyhow!("can't parse"))? as usize).sub(97);

        Ok([file, rank])
    }

    fn verify_any_own_position(&self, positions: Vec<Position>) -> Result<Position> {
        for pos in positions.iter() {
            if self.own_position(pos) {
                return Ok(*pos);
            }
        }
        return Err(anyhow!("Couldn't find [from] position for {:?}", positions));
    }

    fn own_position(&self, pos: &Position) -> bool {
        let piece = self.get_piece(*pos);
        if piece.is_piece() && piece.get_color() == *self.get_active_color() {
            return true;
        }
        false
    }
}
impl Debug for BoardMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            writeln!(f, "{} {:?}", 8 - i, self.squares[i]).unwrap();
        }
        writeln!(f, "   a   b   c   d   e   f   g   h").unwrap();
        writeln!(f, "fen: {}", self.get_fen()).unwrap();
        writeln!(
            f,
            "{}'s turn",
            match self.active_color {
                PieceColor::Black => "black",
                PieceColor::White => "white",
            }
        )
        .unwrap();

        Ok(())
    }
}

impl Deref for BoardMap {
    type Target = [[Piece; 8]; 8];

    fn deref(&self) -> &Self::Target {
        &self.squares
    }
}

impl DerefMut for BoardMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.squares
    }
}
