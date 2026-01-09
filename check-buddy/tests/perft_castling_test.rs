use check_buddy::position_move::PositionMove;
use check_buddy::BoardMap;

fn perft(board_map: BoardMap, depth: usize) -> usize {
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
                let mut next_board = board_map;

                let position_move = PositionMove {
                    from,
                    to,
                    en_passant: board_map.is_en_passant(from, to),
                    promotion: board_map.is_promotion(from, to),
                    ..Default::default()
                };

                next_board.make_move(position_move);
                next_board.switch_active_color();

                num_moves += perft(next_board, depth - 1);
            }
        }
    }

    num_moves
}

#[test]
fn test_perft_castling_position() {
    // Position where both sides can castle
    let board = BoardMap::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

    // At depth 1, should have castling moves available
    let depth1 = perft(board, 1);
    println!("Depth 1: {} moves", depth1);

    // Expected: 26 moves at depth 1
    // - Rooks: 2 rooks * (5+5) moves = 20
    // - King: 5 normal moves + 2 castling moves = 7
    // But let's see what we actually get

    // The standard perft value for this position at depth 1 is 26
    assert_eq!(depth1, 26, "Should have 26 moves at depth 1 with castling available");
}
