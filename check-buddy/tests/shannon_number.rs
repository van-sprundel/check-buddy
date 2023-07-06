use check_buddy::position_move::{Position, PositionMove};
use check_buddy::BoardMap;

const SHANNON_TABLE: [usize; 6] = [20, 400, 8_902, 197_281, 4_865_609, 119_060_324];

#[test]
fn move_integration_test_should_return_valid_move_count_on_depth_one() {
    let board_map = BoardMap::starting();
    assert_eq!(20, move_integration(board_map, 1));
}

#[test]
fn move_integration_test_should_match_shannon_number() {
    //currently layer 5 takes longer than 60 seconds
    assert_eq!(SHANNON_TABLE[3], move_integration(BoardMap::starting(), 4));
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
                        .gen_legal_positions([x, y])
                        .iter()
                        .map(|i| ([x, y], *i))
                        .collect::<Vec<(Position, Position)>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut num_moves = 0;

    for (from, to) in positions {
        let mut board_map = board_map;
        if board_map
            .single_move_turn(PositionMove {
                from,
                to,
                en_passant: false,
                promotion: false,
            })
            .is_ok()
        {
            num_moves += move_integration(board_map, depth - 1);
        }
    }

    num_moves
}
