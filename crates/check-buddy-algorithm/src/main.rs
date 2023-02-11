#![allow(unused)]

use check_buddy_core::piece_move::*;
use check_buddy_core::*;
use rand::Rng;

fn main() {
    let mut chess_engine = ChessEngine::new();
    // println!("{}", chess_engine.mini(, 3));
    // println!("{}", chess_engine.maxi(, 3));
    println!("{:?}", chess_engine.find_best_move(0));
}

// const BLACK: usize = 8;
// const WHITE: usize = 16;

// const PAWN: usize = 1;
// const BISHOP: usize = 2;
// const KNIGHT: usize = 3;
// const ROOK: usize = 4;
// const QUEEN: usize = 5;
// const KING: usize = 6;

#[derive(Clone)]
struct ChessEngine {
    board: BoardMap,
    moves: Vec<PieceMove>,
}

impl ChessEngine {
    const MIN: f32 = -1000.;
    const MAX: f32 = 1000.;

    pub fn new() -> Self {
        Self {
            board: BoardMap::starting(),
            moves: vec![],
        }
    }

    fn maxi(&self, moves: &Vec<PieceMove>, depth: usize) -> f32 {
        if depth == 0 {
            return self.evaluate();
        }

        let mut max = Self::MAX;
        for piece_move in moves.iter() {
            let score = self.mini(moves, depth - 1);
            if score > max {
                max = score;
            }
        }
        max
    }

    fn mini(&self, moves: &Vec<PieceMove>, depth: usize) -> f32 {
        if depth == 0 {
            return -self.evaluate();
        }

        let mut min = Self::MIN;
        for piece_move in moves.iter() {
            let score = self.maxi(moves, depth - 1);
            if score < min {
                min = score;
            }
        }
        min
    }

    fn nega_max(&mut self, from: Position, depth: usize) -> f32 {
        if depth == 0 {
            return self.evaluate();
        }
        let mut max = Self::MIN;
        // let moves = if self.board.get_active_color() == &PieceColor::Black {
        //     self.board.gen_opponent_moves()
        // } else {
        //     self.board.gen_all_legal_moves()
        // };
        let moves = self.board.gen_all_legal_moves();
        for to in moves.iter() {
            let piece_move = PieceMove::new(from, *to);
            // MAKE MOVE
            let mut temp_engine = self.clone(); //TODO find cheaper way
            temp_engine.board.move_turn(piece_move);

            let value = -temp_engine.nega_max(from, depth - 1);
            if value > max {
                max = value;
            }
        }
        max
    }
    /// score = materialWeight * (numWhitePieces - numBlackPieces) * who2move
    fn evaluate(&self) -> f32 {
        let material_weight = self.board.get_material_weight() as f32;
        let num_white_pieces = self.board.get_num_white_pieces() as f32;
        let num_black_pieces = self.board.get_num_black_pieces() as f32;
        let who2move = if *self.board.get_active_color() == PieceColor::Black {
            1.
        } else {
            -1.
        };
        // println!("{material_weight} * ({num_white_pieces} - {num_black_pieces}) * {who2move}");
        material_weight * (num_white_pieces - num_black_pieces) * who2move
    }

    fn find_best_move(&mut self, depth: usize) -> PieceMove {
        let mut best_value = Self::MIN;
        let mut best_moves = vec![];
        for from in self.board.get_active_pieces() {
            let moves = self.board.gen_legal_moves(from);
            for to in moves {
                let piece_move = PieceMove::new(from, to);
                let piece_value = self.board.get_piece(from).0;
                let mut temp_engine = self.clone();
                temp_engine.board.move_turn(piece_move);
                let move_value = temp_engine.nega_max(from, depth);
                temp_engine.board.undo_move(piece_move, piece_value);

                if move_value == best_value {
                    best_moves.push(piece_move);
                } else if move_value > best_value {
                    best_value = move_value;
                    best_moves.clear();
                    best_moves.push(piece_move);
                }
            }
        }

        println!("best move value is {}", best_value);
        println!(
            "possible moves are {:?}",
            best_moves
                .iter()
                .take(5)
                .map(|x| format!("{:?} to {:?}", x.from, x.to))
                .collect::<Vec<_>>()
        );

        let mut rand = rand::thread_rng();
        let index = rand.gen_range(0..best_moves.len());
        return best_moves[index].clone();
    }
}
