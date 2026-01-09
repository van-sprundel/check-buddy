use check_buddy::BoardMap;

#[test]
fn test_castling_allowed_at_start() {
    // from starting position, no castling possible
    let board = BoardMap::starting();

    let white_king_moves = board.gen_legal_positions([7, 4]);
    let black_king_moves = board.gen_legal_positions([0, 4]);

    println!("White king legal moves: {:?}", white_king_moves);
    println!("Black king legal moves: {:?}", black_king_moves);

    // kings can't castle at start
    assert!(!white_king_moves.contains(&[7, 6])); // no kingside castle
    assert!(!white_king_moves.contains(&[7, 2])); // no queenside castle
    assert!(!black_king_moves.contains(&[0, 6])); // no kingside castle
    assert!(!black_king_moves.contains(&[0, 2])); // no queenside castle
}

#[test]
fn test_castling_allowed_when_clear() {
    // position where white can castle kingside
    let board = BoardMap::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

    let white_king_moves = board.gen_legal_positions([7, 4]);
    let black_king_moves = board.gen_legal_positions([0, 4]);

    println!("White king moves (can castle): {:?}", white_king_moves);
    println!("Black king moves (can castle): {:?}", black_king_moves);

    assert!(
        white_king_moves.contains(&[7, 6]),
        "White should be able to castle kingside"
    );
    assert!(
        white_king_moves.contains(&[7, 2]),
        "White should be able to castle queenside"
    );
    assert!(
        black_king_moves.contains(&[0, 6]),
        "Black should be able to castle kingside"
    );
    assert!(
        black_king_moves.contains(&[0, 2]),
        "Black should be able to castle queenside"
    );
}

#[test]
fn test_en_passant_generates_moves() {
    // white pawn on e5, black pawn just moved from d7 to d5
    let mut board = BoardMap::from_fen("8/8/8/3pP3/8/8/8/8 w - - 0 1");

    // the black pawn at d5 should be marked as en-passantable (pawn(true))
    use check_buddy::piece_type::*;
    board.set_piece([3, 3], PAWN | BLACK | 32);

    let white_pawn_moves = board.gen_legal_positions([3, 4]); // e5

    println!("white pawn at e5 moves: {:?}", white_pawn_moves);

    assert!(
        white_pawn_moves.contains(&[2, 3]),
        "Should be able to capture en passant"
    );
}

#[test]
fn test_perft_specific_depth() {
    let board = BoardMap::starting();

    let mut depth1_count = 0;
    for x in 0..8 {
        for y in 0..8 {
            let from = [x, y];
            let piece = board.get_piece(from);
            if piece.is_piece() && piece.get_color() == *board.get_active_color() {
                let moves = board.gen_legal_positions(from);
                depth1_count += moves.len();
                if !moves.is_empty() {
                    println!("piece at {:?} has {} moves: {:?}", from, moves.len(), moves);
                }
            }
        }
    }

    println!("moves at depth 1: {}", depth1_count);
    assert_eq!(depth1_count, 20, "Should have exactly 20 moves at depth 1");
}
