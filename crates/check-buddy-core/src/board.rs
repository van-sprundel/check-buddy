use crate::piece::piece_move::{
    Direction, PieceMove, Position, DIRECTION_OFFSETS, KNIGHT_DIRECTION_OFFSETS,
};
use crate::piece::{piece_type::*, Piece, PieceColor};
use std::borrow::BorrowMut;
use std::cmp::min;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

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
    pub fn starting() -> Self {
        BoardMap::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
    /// starting position: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    /// white is uppercase, black is lowercase
    pub fn from_fen(fen: &str) -> Self {
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
                    board.squares[7 - (index / 8)][index % 8] = Piece(color | rank);
                    index += 1;
                } else {
                    index += x.to_digit(10).unwrap() as usize;
                }
            }
        });
        board.active_color = if let Some(c) = sections[1].chars().next() {
            match c {
                'w' => PieceColor::White,
                'b' => PieceColor::Black,
                _ => unreachable!("FEN incorrect"),
            }
        } else {
            PieceColor::White
        };

        board
    }
    pub fn get_fen(&self) -> String {
        todo!()
    }
    pub fn get_piece(&self, pos: Position) -> Piece {
        self.squares[pos[0]][pos[1]]
    }
    pub fn find_piece(&self, piece_color: PieceColor, piece_type: PieceType) -> Option<Position> {
        for (y, row) in self.squares.iter().enumerate() {
            for (x, p) in row.iter().enumerate() {
                if let Some(t) = p.get_type() {
                    if t == piece_type && p.get_color() == piece_color {
                        return Some([y, x]);
                    }
                }
            }
        }
        None
    }
    pub fn get_piece_mut(&mut self, pos: Position) -> &mut Piece {
        self.squares[pos[0]][pos[1]].borrow_mut()
    }
    pub fn get_active_color(&self) -> &PieceColor {
        &self.active_color
    }
    fn set_piece(&mut self, on: Position, value: u32) {
        self.squares[on[0]][on[1]] = Piece(value);
    }

    /// make a move with check
    /// returns true if move was successful
    pub fn move_turn(&mut self, piece_move: PieceMove) -> bool {
        let PieceMove { from, to, .. } = piece_move;
        let (x, y) = (from[0], from[1]);
        let piece = self.squares[x][y];
        if !piece.is_piece() {
            // eprintln!("You're trying to move a piece that's empty");
            return false;
        }
        // if (piece.is_white() && !self.active_color) || (piece.is_black() && self.active_color) {
        if piece.get_color() == self.active_color {
            let should_enable_en_passant = self.move_should_enable_en_passant(piece_move);
            if self.is_valid_move(piece_move) {
                let en_passant = self.is_en_passant(piece_move);
                let trade = self.is_trade(piece_move);

                self.make_move(PieceMove {
                    from: [x, y],
                    to,
                    en_passant,
                    trade,
                });

                let piece_to = &mut self.get_piece(to);

                if let Some(PieceType::Pawn(_)) = piece_to.get_type() {
                    if should_enable_en_passant {
                        // eprintln!("Piece became en passantable! ({},{})", to[0], to[1]);
                        self.get_piece_mut(to).0 += 32;
                    } else if self.get_piece(to).0 > 32 {
                        self.get_piece_mut(to).0 -= 32;
                    }
                }

                self.switch_active_color();
                return true;
            } else {
                // eprintln!("Move was invalid");
            }
        } else {
            // eprintln!("Piece is not yours");
        }
        false
    }
    /// check if move is valid
    pub fn is_valid_move(&self, piece_move: PieceMove) -> bool {
        let PieceMove { from, to, .. } = piece_move;

        let piece_to = self.squares[to[0]][to[1]];

        if !piece_to.is_piece() || piece_to.get_color() != self.active_color {
            let moves = self.gen_legal_moves(from);
            return moves.contains(&to);
        }
        false
    }
    pub fn is_hit(&self, pos: Position) -> bool {
        let piece_on = self.get_piece(pos);
        piece_on.is_piece() && piece_on.get_color() != self.active_color
    }
    /// generate only legal moves for piece
    pub fn gen_legal_moves(&self, from: Position) -> Vec<Position> {
        let mut temp_board = *self;
        let moves = temp_board.gen_moves(from);
        let mut legal_moves = vec![];
        // eprintln!("moves {:?}", &moves);

        for to in moves.into_iter() {
            let last_piece = temp_board.squares[to[0]][to[1]].0;

            temp_board.make_move(PieceMove {
                from,
                to,
                en_passant: false,
                trade: false,
            });
            let next_moves = temp_board.gen_opponent_moves();
            // eprintln!("next possible moves: {:?}", next_moves);
            if !next_moves.iter().any(|x| {
                let next_piece = temp_board.squares[x[0]][x[1]];
                if next_piece.is_piece() && next_piece.get_color() == temp_board.active_color {
                    // eprintln!("{:?}", next_piece);
                    if let Some(t) = next_piece.get_type() {
                        return t == PieceType::King;
                    }
                }
                false
            }) {
                legal_moves.push(to);
            }

            temp_board.undo_move(
                PieceMove {
                    from,
                    to,
                    en_passant: false,
                    trade: false,
                },
                last_piece,
            );
        }
        // eprintln!("legal moves {:?}", &legal_moves);
        legal_moves
    }
    /// generate all possible moves for piece
    pub fn gen_moves(&self, from: Position) -> Vec<Position> {
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
    fn gen_sliding(&self, from: Position, piece_type: PieceType) -> Vec<Position> {
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
                    // eprintln!("Piece is yours!");
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
    fn gen_king(&self, from: Position) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        let mut moves = vec![];
        for direction in DIRECTION_OFFSETS {
            let index = from[0] * 8 + from[1];
            let target_index = index as i32 + direction;
            if !(0..=63).contains(&target_index) {
                continue;
            }
            let target_move = [target_index as usize / 8, target_index as usize % 8];
            let target_piece = self.squares[target_move[0]][target_move[1]];

            if target_piece.is_piece() && target_piece.get_color() == piece_from.get_color() {
                // your own color is in the way
                // eprintln!("Piece is yours!");
                continue;
            }
            moves.push(target_move);
            // self.squares[target_move[0]][target_move[1]] = Piece(100);

            if target_piece.is_piece() && target_piece.get_color() != piece_from.get_color() {
                // Enemy piece and capturable
                // eprintln!("Piece is not yours, but you should still break the loop!");
                continue;
            }
        }
        moves
    }
    fn gen_pawn(&self, from: Position) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        let mut moves = vec![];
        let shift = match piece_from.get_color() {
            PieceColor::Black => -1,
            PieceColor::White => 1,
        };

        // piece blocking
        let is_blocking = self.squares[(from[0] as i32 + shift) as usize][from[1]].is_piece();
        if !is_blocking {
            moves.push([(from[0] as i32 + shift) as usize, from[1]]);
        }

        // hasn't moved yet
        let vertical = (from[0] as i32 + shift * 2) as usize;
        if vertical < 8 {
            let is_blocking = is_blocking || self.squares[vertical][from[1]].is_piece();
            if ((piece_from.is_black() && from[0] == 6) || (piece_from.is_white() && from[0] == 1))
                && !is_blocking
            {
                moves.push([vertical, from[1]]);
            }
        }

        // takeable pieces on [+1,-1]
        // x  .  .
        // .  p  .
        if (1..8).contains(&from[1]) {
            let to_top_left_pos = [(from[0] as i32 + shift) as usize, from[1] - 1];
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

        // takeable pieces on [+1,+1]
        // .  .  x
        // .  p  .
        if (0..7).contains(&from[1]) {
            let to_top_right_pos = [(from[0] as i32 + shift) as usize, from[1] + 1];
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
        moves
    }
    fn gen_knight(&self, from: Position) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        let mut moves = vec![];
        for direction in KNIGHT_DIRECTION_OFFSETS {
            let new_pos = [direction[0] + from[0] as i32, direction[1] + from[1] as i32];
            if (0..8).contains(&new_pos[0]) && (0..8).contains(&new_pos[1]) {
                let to_move = [new_pos[0] as usize, new_pos[1] as usize];
                let target_piece = self.squares[to_move[0]][to_move[1]];
                if target_piece.is_piece() && target_piece.get_color() == piece_from.get_color() {
                    continue;
                }

                moves.push(to_move);
            }
        }

        moves
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
    fn make_move(&mut self, piece_move: PieceMove) {
        let PieceMove {
            from,
            to,
            en_passant,
            trade,
        } = piece_move;

        if en_passant {
            // eprintln!("move is an en passant!");
            let shift = if self.get_piece(from).get_color() == PieceColor::Black {
                -1
            } else {
                1
            };
            let to_step = [(to[0] as isize + shift) as usize, to[1]];
            self.set_piece(to_step, BLACK);
        }

        if trade {
            let color = match self.get_piece(from).get_color() {
                PieceColor::Black => BLACK,
                PieceColor::White => WHITE,
            };
            self.set_piece(to, QUEEN | color);
        } else {
            self.set_piece(to, self.get_piece(from).0);
        }
        self.set_piece(
            from,
            if (from[0] + from[1]) % 2 != 0 {
                BLACK
            } else {
                WHITE
            },
        );
    }
    fn undo_move(&mut self, piece_move: PieceMove, last_piece: u32) {
        let PieceMove { from, to, .. } = piece_move;
        self.set_piece(from, self.get_piece(to).0);
        self.set_piece(to, last_piece);
    }
    fn gen_opponent_moves(&self) -> Vec<Position> {
        let mut opponent_moves = vec![];
        for rank in 0..8 {
            for file in 0..8 {
                let piece = self.squares[rank][file];
                if piece.is_piece() && piece.get_color() != self.active_color {
                    // eprintln!("found enemy piece! {:?}", piece);
                    let moves = self.gen_moves([rank, file]);
                    opponent_moves.extend(moves);
                }
            }
        }
        opponent_moves
    }

    fn switch_active_color(&mut self) {
        self.active_color = if self.active_color == PieceColor::Black {
            PieceColor::White
        } else {
            PieceColor::Black
        };
    }

    fn move_should_enable_en_passant(&self, piece_move: PieceMove) -> bool {
        let PieceMove { from, to, .. } = piece_move;
        let piece = self.get_piece(from);
        if let Some(PieceType::Pawn(_)) = piece.get_type() {
            if *self.get_active_color() == piece.get_color() {
                return match piece.get_color() {
                    PieceColor::Black => from[0] == 6 && to[0] == 4,
                    PieceColor::White => from[0] == 1 && to[0] == 3,
                };
            }
        }
        false
    }

    pub fn is_en_passant(&self, piece_move: PieceMove) -> bool {
        // only en passant moves can be moved diagonally on an empty square
        let PieceMove { from, to, .. } = piece_move;
        let piece = self.get_piece(from);
        if let Some(piece_type) = piece.get_type() {
            if piece_type == PieceType::Pawn(false) || piece_type == PieceType::Pawn(true) {
                let shift = match piece.get_color() {
                    PieceColor::Black => -1,
                    PieceColor::White => 1,
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
    pub fn is_trade(&self, piece_move: PieceMove) -> bool {
        let PieceMove { from, to, .. } = piece_move;
        let piece = self.get_piece(from);
        if let Some(piece_type) = piece.get_type() {
            return (piece_type == PieceType::Pawn(false) || piece_type == PieceType::Pawn(true))
                && (to[0] == 7 || to[0] == 0);
        }

        false
    }
}

impl Debug for BoardMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..8 {
            writeln!(f, "{} {:?}", 8 - x, self.squares[7 - x]).unwrap();
        }
        writeln!(f, "   a   b   c   d   e   f   g   h").unwrap();
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
