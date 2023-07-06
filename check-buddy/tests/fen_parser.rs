use check_buddy_core::piece_type::{BISHOP, BLACK, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE};
use check_buddy_core::position_move::Position;
use check_buddy_core::{BoardMap, Piece};

#[test]
fn fen_to_board() {
    let board = BoardMap::from_fen("4k3/8/8/8/8/8/8/3K4 w - - 0 1");

    assert_piece(board, [0, 4], KING | BLACK);
    assert_piece(board, [7, 3], KING | WHITE);
}

#[test]
fn complex_fen_to_board() {
    let board = BoardMap::from_fen("4k2N/1qq5/p4b2/1B6/3R2B1/1n2BqP1/8/2QK2r1");
    assert_piece(board, [0, 4], KING | BLACK);
    assert_piece(board, [0, 7], KNIGHT | WHITE);
    assert_piece(board, [1, 1], QUEEN | BLACK);
    assert_piece(board, [1, 2], QUEEN | BLACK);
    assert_piece(board, [2, 0], PAWN | BLACK);
    assert_piece(board, [2, 5], BISHOP | BLACK);
    assert_piece(board, [3, 1], BISHOP | WHITE);
    assert_piece(board, [4, 3], ROOK | WHITE);
    assert_piece(board, [4, 6], BISHOP | WHITE);
    assert_piece(board, [5, 1], KNIGHT | BLACK);
    assert_piece(board, [5, 4], BISHOP | WHITE);
    assert_piece(board, [5, 5], QUEEN | BLACK);
    assert_piece(board, [5, 6], PAWN | WHITE);
    assert_piece(board, [7, 2], QUEEN | WHITE);
    assert_piece(board, [7, 3], KING | WHITE);
    assert_piece(board, [7, 6], ROOK | BLACK);
}

#[test]
fn board_to_fen() {
    let eventual_fen = "4k2N/1qq5/p4b2/1B6/3R2B1/1n2BqP1/8/2QK2r1";
    let mut board = BoardMap::empty();
    board.set_piece([0, 4], KING | BLACK);
    board.set_piece([0, 7], KNIGHT | WHITE);
    board.set_piece([1, 1], QUEEN | BLACK);
    board.set_piece([1, 2], QUEEN | BLACK);
    board.set_piece([2, 0], PAWN | BLACK);
    board.set_piece([2, 5], BISHOP | BLACK);
    board.set_piece([3, 1], BISHOP | WHITE);
    board.set_piece([4, 3], ROOK | WHITE);
    board.set_piece([4, 6], BISHOP | WHITE);
    board.set_piece([5, 1], KNIGHT | BLACK);
    board.set_piece([5, 4], BISHOP | WHITE);
    board.set_piece([5, 5], QUEEN | BLACK);
    board.set_piece([5, 6], PAWN | WHITE);
    board.set_piece([7, 2], QUEEN | WHITE);
    board.set_piece([7, 3], KING | WHITE);
    board.set_piece([7, 6], ROOK | BLACK);
    let generated_fen = board.get_fen();
    assert_eq!(eventual_fen, generated_fen);
}

fn assert_piece(board: BoardMap, pos: Position, piece_value: u32) {
    let piece = board.get_piece(pos);
    assert_eq!(Piece(piece_value), piece);
}
