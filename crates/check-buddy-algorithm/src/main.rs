#![allow(unused)]

use check_buddy_core::piece_move::*;
use check_buddy_core::*;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut chess_engine = ChessEngine::new();
    chess_engine.board = BoardMap::starting();

    println!("{:?}", chess_engine.find_best_move_minimax_ab(0));
    println!("-----");
    println!("{:?}", chess_engine.find_best_move_minimax_ab(1));
    println!("-----");
    // println!("{:?}", chess_engine.find_best_move_minimax_ab(6));
}

#[derive(Clone)]
struct ChessEngine {
    board: BoardMap,
}

impl ChessEngine {
    const MIN: f32 = -1000.;
    const MAX: f32 = 1000.;

    pub fn new() -> Self {
        Self {
            board: BoardMap::starting(),
        }
    }

    fn ab_max(
        &self,
        depth: usize,
        mut alpha: f32,
        mut beta: f32,
        mut is_maximizing_player: bool,
    ) -> f32 {
        return if is_maximizing_player {
            // AB MAX
            if depth == 0 {
                return self.evaluate();
            }
            let moves = self.board.gen_all_legal_moves();

            for piece_move in moves.iter() {
                let mut temp_engine = self.clone(); //TODO find cheaper way
                temp_engine.board.move_turn(*piece_move);

                let score = temp_engine.ab_max(depth - 1, alpha, beta, !is_maximizing_player);
                if score >= beta {
                    return beta;
                }
                if score > alpha {
                    alpha = score;
                }
            }
            alpha
        } else {
            // AB MIN
            if depth == 0 {
                return -self.evaluate();
            }
            let moves = self.board.gen_all_legal_moves();

            for piece_move in moves.iter() {
                let mut temp_engine = self.clone(); //TODO find cheaper way
                temp_engine.board.move_turn(*piece_move);

                let score = temp_engine.ab_max(depth - 1, alpha, beta, !is_maximizing_player);
                if score <= alpha {
                    return alpha;
                }
                if score < beta {
                    beta = score;
                }
            }
            beta
        };
    }

    fn nega_max(&mut self, depth: usize) -> f32 {
        if depth == 0 {
            return self.evaluate();
        }
        let mut max = Self::MIN;
        let moves = self.board.gen_all_legal_moves();

        for piece_move in moves.iter() {
            let mut temp_engine = self.clone(); //TODO find cheaper way
            temp_engine.board.move_turn(*piece_move);

            let value = -temp_engine.nega_max(depth - 1);
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

    fn find_best_move_minimax_ab(&mut self, depth: usize) -> PieceMove {
        let mut best_value = Self::MIN;
        let mut best_moves = vec![];
        for from in self.board.get_active_pieces() {
            let moves = self.board.gen_legal_moves(from);
            for to in moves {
                let piece_move = PieceMove::new(from, to);
                let piece_value = self.board.get_piece(from).0;
                let mut temp_engine = self.clone();
                temp_engine.board.move_turn(piece_move);
                let move_value =
                    temp_engine.ab_max(depth, ChessEngine::MIN, ChessEngine::MAX, true);
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
                .map(|x| format!("{:?} to {:?}", x.from, x.to))
                .collect::<Vec<_>>()
        );

        let mut rand = rand::thread_rng();
        let index = rand.gen_range(0..best_moves.len());
        best_moves[index]
    }

    fn find_best_move_negamax(&mut self, depth: usize) -> PieceMove {
        let mut best_value = Self::MIN;
        let mut best_moves = vec![];
        for from in self.board.get_active_pieces() {
            let moves = self.board.gen_legal_moves(from);
            for to in moves {
                let piece_move = PieceMove::new(from, to);
                let piece_value = self.board.get_piece(from).0;
                let mut temp_engine = self.clone();
                temp_engine.board.move_turn(piece_move);
                let move_value = temp_engine.nega_max(depth);
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
                .take(10)
                .map(|x| format!("{:?} to {:?}", x.from, x.to))
                .collect::<Vec<_>>()
        );

        let mut rand = rand::thread_rng();
        let index = rand.gen_range(0..best_moves.len());
        best_moves[index]
    }
}
