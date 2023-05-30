use std::env;
use anyhow::Result;

use check_buddy_core::position_move::{Position, PositionMove};
use check_buddy_core::BoardMap;

#[test]
fn uci_moves_should_be_valid() -> Result<()> {
    let move_data = gen_move_data().unwrap();
    for (row, (id, move_name, moves)) in move_data.iter().skip(10294).enumerate() {
        let mut board = BoardMap::starting();
        for piece_move in moves {
            let actual_move = board
                .parse_uci_to_move(piece_move)
                .expect(&*format!("Game {}: ({})", row, move_name));
            let PositionMove { from, to, .. } = actual_move.1;
            let positions = (0..8)
                .flat_map(|x| {
                    (0..8)
                        .flat_map(|y| {
                            board
                                .gen_legal_positions([x, y])
                                .iter()
                                .map(|i| ([x, y], *i))
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<(Position, Position)>>();

            if !positions
                .iter()
                .any(|(piece_from, piece_to)| *piece_from == from && *piece_to == to)
            {
                let piece = board.get_piece(from);
                panic!("row {row}
    {board:?}
    Game {id}: ({move_name})
    Move {piece_move} is invalid
    Moving piece {piece:?} from {from:?} to {to:?} isn't seen as a valid move
");
            }
            board.uci_move_turn(actual_move)?;
        }
    }
    Ok(())
}

fn gen_move_data() -> Result<Vec<(String, String, Vec<String>)>> {
    let path = format!("{}/tests/datasets/games.csv", env!("CARGO_MANIFEST_DIR"));
    let file = std::fs::File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    Ok(rdr
        .records()
        .map(|result| {
            let record = result.expect("couldn't parse record");
            let moves = &record[12];
            let moves = moves.split(' ').map(|s| s.to_owned()).collect::<Vec<_>>();
            let id = &record[0];
            let opening_name = &record[14];
            (id.to_owned(), opening_name.to_owned(), moves)
        })
        .collect::<Vec<_>>())
}
