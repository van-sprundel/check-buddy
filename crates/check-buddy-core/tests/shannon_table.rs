use check_buddy_core::board::BoardMap;
use check_buddy_core::piece::piece_move::{PieceMove, Position};

#[test]
fn move_integration_test_should_return_valid_move_count_on_depth_one() {
    let board_map = BoardMap::starting();
    assert_eq!(20, move_integration(board_map, 1));
}

#[test]
fn move_integration_test_should_match_shannon_table() {
    let board_map = BoardMap::starting();

    assert_eq!(20, move_integration(board_map, 1));
    assert_eq!(400, move_integration(board_map, 2));
    assert_eq!(8902, move_integration(board_map, 3));
    assert_eq!(197281, move_integration(board_map, 4));
}

fn move_integration(board_map: BoardMap, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let positions = (0..8)
        .flat_map(|x| {
            (0..8)
                .flat_map(|y| {
                    board_map
                        .gen_legal_moves([x, y])
                        .iter()
                        .map(|i| ([x, y], *i))
                        .collect::<Vec<(Position, Position)>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut num_moves = 0;

    for (from, to) in positions {
        let mut board_map = board_map.clone();
        if board_map.move_turn(PieceMove {
            from,
            to,
            en_passant: false,
            trade: false,
        }) {
            num_moves += move_integration(board_map, depth - 1);
        }
    }

    num_moves
}
