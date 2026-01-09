use check_buddy::position_move::PositionMove;
use check_buddy::BoardMap;

const SHANNON_TABLE: [usize; 6] = [20, 400, 8_902, 197_281, 4_865_609, 119_060_324];

#[test]
fn move_integration_test_should_return_valid_move_count_on_depth_one() {
    let board_map = BoardMap::starting();
    assert_eq!(20, move_integration(board_map, 1));
}

#[test]
fn move_integration_test_should_match_shannon_number() {
    let board_map = BoardMap::starting();
    for depth in 1..=5 {
        let result = move_integration(board_map, depth);
        assert_eq!(
            SHANNON_TABLE[depth - 1],
            result,
            "Mismatch at depth {}: expected {}, got {}",
            depth,
            SHANNON_TABLE[depth - 1],
            result
        );
    }
}

fn move_integration(board_map: BoardMap, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut num_moves = 0;
    for x in 0..8 {
        for y in 0..8 {
            let from = [x, y];
            let piece = board_map.get_piece(from);
            if !piece.is_piece() || piece.get_color() != *board_map.get_active_color() {
                continue;
            }
            for to in board_map.gen_legal_positions(from) {
                let is_en_passant = board_map.is_en_passant(from, to);
                let is_promotion = board_map.is_promotion(from, to);
                if is_promotion {
                    // Generate all 4 promotion options
                    use check_buddy::piece_type::{BISHOP, KNIGHT, QUEEN, ROOK};
                    for promotion_piece in [QUEEN, ROOK, BISHOP, KNIGHT] {
                        let mut next_board = board_map;
                        let position_move = PositionMove {
                            from,
                            to,
                            en_passant: is_en_passant,
                            promotion: true,
                            promotion_piece,
                        };
                        next_board.make_move(position_move);
                        next_board.switch_active_color();
                        num_moves += move_integration(next_board, depth - 1);
                    }
                } else {
                    let mut next_board = board_map;
                    let position_move = PositionMove {
                        from,
                        to,
                        en_passant: is_en_passant,
                        promotion: false,
                        ..Default::default()
                    };
                    next_board.make_move(position_move);
                    next_board.switch_active_color();
                    num_moves += move_integration(next_board, depth - 1);
                }
            }
        }
    }
    num_moves
}
