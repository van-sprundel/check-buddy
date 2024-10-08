use anyhow::Result;
use std::env;

use check_buddy::position_move::PositionMove;
use check_buddy::BoardMap;

#[test]
fn uci_moves_should_be_valid() -> Result<()> {
    let move_data = gen_move_data().unwrap();
    for (row, (id, move_name, moves)) in move_data.iter().enumerate() {
        let mut board = BoardMap::starting();
        for piece_move in moves {
            let actual_move = board.parse_uci_to_move(piece_move).unwrap_or_else(|_| {
                panic!(
                    "
    Row {row}
    Game {id}: ({move_name})
"
                )
            });
            let uci_move = actual_move.0;
            let PositionMove { from, to, .. } = actual_move.1;
            let positions = board.gen_legal_positions(from);

            if !positions.iter().any(|piece_to| *piece_to == to) {
                let piece = board.get_piece(from);
                panic!(
                    "
{board:?}
    row {row}
    Game {id}: ({move_name})
    Uci {uci_move:?} is invalid
    Move {piece_move} is invalid
    Moving piece {piece:?} from {from:?} to {to:?} isn't seen as a valid move
    Legal positions: {positions:?}
"
                );
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
