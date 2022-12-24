use check_buddy_core::piece_move::{PieceMove, Position};
use check_buddy_core::BoardMap;

const SHANNON_TABLE: [usize; 5] = [20, 400, 8902, 197281, 4865609];

#[test]
fn move_integration_test_should_return_valid_move_count_on_depth_one() {
    let board_map = BoardMap::starting();
    assert_eq!(20, move_integration(board_map, 1));
}

#[test]
fn move_integration_test_should_match_shannon_number() {
    //currently layer 5 takes longer than 60 seconds
    for depth in 1..=4 {
        assert_eq!(
            SHANNON_TABLE[depth - 1],
            move_integration(BoardMap::starting(), depth)
        );
    }
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
        if board_map
            .move_turn(PieceMove {
                from,
                to,
                en_passant: false,
                trade: false,
            })
            .is_ok()
        {
            num_moves += move_integration(board_map, depth - 1);
        }
    }

    num_moves
}
