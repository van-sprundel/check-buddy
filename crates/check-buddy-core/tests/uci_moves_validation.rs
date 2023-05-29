use std::env;
use anyhow::Result;

use check_buddy_core::position_move::{Position, PositionMove};
use check_buddy_core::BoardMap;

#[test]
fn uci_moves_should_be_valid() -> Result<()> {
    let move_data = gen_move_data().unwrap();
    for (row, (id, move_name, moves)) in move_data.iter().skip(97).enumerate() {
        let mut board = BoardMap::starting();
        for piece_move in moves {
            println!("piece move {:?}", piece_move);
            println!("BEGIN -------------\n{:?}", board);
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
                println! {"{:?}", board};
                println!("Game {}: ({})", id, move_name);
                println!("move {} is invalid", piece_move);
                println!(
                    "Moving piece {:?} from {:?} to {:?} isn't seen as a valid move",
                    piece, from, to
                );
                panic!();
            }
            println!("actual move {:?}", actual_move.1);
            board.uci_move_turn(actual_move)?;
            println!("END ---------------\n{:?}", board);
        }
    }
    Ok(())
}

fn gen_move_data() -> Result<Vec<(String, String,Vec<String>)>> {
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
