#![allow(unused_must_use)]
use crate::piece_color::PieceColor;
use crate::position_move::PositionMove;
use crate::BoardMap;
use rand::Rng;

#[derive(Default, Clone)]
pub struct ChessEngine {}

impl ChessEngine {
    const MIN: f32 = -1000.;
    const MAX: f32 = 1000.;

    pub fn find_best_move_minimax_ab(&mut self, board: BoardMap, depth: usize) -> PositionMove {
        let mut best_value = Self::MIN;
        let mut best_moves = vec![];
        for from in board.get_active_pieces() {
            let moves = board.gen_legal_positions(from);
            for to in moves {
                let piece_move = PositionMove::new(from, to);
                let piece_value = board.get_piece(from).0;
                let mut temp_board = board;

                temp_board.single_move_turn(piece_move);
                let move_value =
                    self.ab_max(depth, temp_board, ChessEngine::MIN, ChessEngine::MAX, true);
                temp_board.undo_move(piece_move, piece_value);

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

    pub fn find_best_move_negamax(&mut self, board: BoardMap, depth: usize) -> PositionMove {
        let mut best_value = Self::MIN;
        let mut best_moves = vec![];
        for from in board.get_active_pieces() {
            let moves = board.gen_legal_positions(from);
            for to in moves {
                let piece_move = PositionMove::new(from, to);
                let piece_value = board.get_piece(from).0;

                let mut temp_board = board;
                temp_board.single_move_turn(piece_move);
                let move_value = self.nega_max(temp_board, depth);
                temp_board.undo_move(piece_move, piece_value);

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

    fn ab_max(
        &self,
        depth: usize,
        board: BoardMap,
        mut alpha: f32,
        mut beta: f32,
        is_maximizing_player: bool,
    ) -> f32 {
        if is_maximizing_player {
            // AB MAX
            if depth == 0 {
                return self.evaluate(board);
            }
            let moves = board.gen_all_legal_moves();

            for piece_move in moves.iter() {
                let mut temp_board = board;

                let piece_value = board.get_piece(piece_move.from).0;

                temp_board.single_move_turn(*piece_move);
                let score = self.ab_max(depth - 1, temp_board, alpha, beta, !is_maximizing_player);
                temp_board.undo_move(*piece_move, piece_value);

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
                return -self.evaluate(board);
            }
            let moves = board.gen_all_legal_moves();

            for piece_move in moves.iter() {
                let mut temp_board = board;

                let piece_value = board.get_piece(piece_move.from).0;

                temp_board.single_move_turn(*piece_move);
                let score = self.ab_max(depth - 1, temp_board, alpha, beta, !is_maximizing_player);
                temp_board.undo_move(*piece_move, piece_value);

                if score <= alpha {
                    return alpha;
                }
                if score < beta {
                    beta = score;
                }
            }
            beta
        }
    }

    fn nega_max(&mut self, board: BoardMap, depth: usize) -> f32 {
        if depth == 0 {
            return self.evaluate(board);
        }
        let mut max = Self::MIN;
        let moves = board.gen_all_legal_moves();

        for piece_move in moves.iter() {
            let mut temp_board = board;
            temp_board.single_move_turn(*piece_move);

            let value = -self.nega_max(board, depth - 1);
            if value > max {
                max = value;
            }
        }
        max
    }

    /// score = materialWeight * (numWhitePieces - numBlackPieces) * who2move
    //TODO https://chessfox.com/example-of-the-complete-evaluation-process-of-chess-a-chess-position/
    // https://www.chessprogramming.org/Evaluation
    //
    fn evaluate(&self, board: BoardMap) -> f32 {
        let material_weight = board.get_material_weight() as f32;
        let num_white_pieces = board.get_num_white_pieces() as f32;
        let num_black_pieces = board.get_num_black_pieces() as f32;
        let who2move = if *board.get_active_color() == PieceColor::Black {
            1.
        } else {
            -1.
        };
        material_weight * (num_white_pieces - num_black_pieces) * who2move
    }
}
