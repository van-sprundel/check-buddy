use anyhow::Result;

use check_buddy_core::piece_move::{PieceMove, Position};
use check_buddy_core::BoardMap;

#[test]
fn opening_move_should_be_valid() {
    let move_data = gen_move_data().unwrap();
    move_data.iter().for_each(|(moves, move_name)| {
        let mut board = BoardMap::starting();
        for piece_move in moves {
            let actual_move = board.parse_move(piece_move).expect(&*format!("Couldn't parse move {}", piece_move));
            let PieceMove{from, to, ..} = actual_move;
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
                panic!(
                    "move {} invalid. \nMoving piece {:?} from {:?} to {:?} isn't seen as a valid move",
                    move_name, piece, from, to
                );
            }
            board.move_turn(actual_move).unwrap();
        }
    });
}

fn gen_move_data() -> Result<Vec<(Vec<String>, String)>> {
    let path = format!("{}/tests/datasets/games.csv", env!("CARGO_MANIFEST_DIR"));
    let file = std::fs::File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    Ok(rdr
        .records()
        .map(|result| {
            let record = result.expect("couldn't parse record");
            let moves = &record[12];
            let moves = moves.split(' ').map(|s| s.to_owned()).collect::<Vec<_>>();
            let opening_name = &record[14];

            (moves, opening_name.to_owned())
        })
        .collect::<Vec<_>>())
}
