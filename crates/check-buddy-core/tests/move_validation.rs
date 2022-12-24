use std::ops::Sub;
use calamine::*;
use check_buddy_core::BoardMap;
use check_buddy_core::piece_move::{PieceMove, Position};
use anyhow::{Result, Error, anyhow};

#[test]
fn best_move_should_be_valid() {
    let move_data = gen_move_data().unwrap();
    move_data.iter().for_each(|(board_map, best_move)| {
        let (best_from, best_to) = best_move;
        let positions = (0..8)
            .flat_map(|x| {
                (0..8)
                    .flat_map(|y| {
                        board_map
                            .gen_legal_moves([x, y])
                            .iter()
                            .map(|i| ([x, y], *i))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<(Position, Position)>>();

        if !positions.iter().any(|(from, to)| from == best_from && to == best_to) {
            let piece = board_map.get_piece(*best_from);
            println! {"{:?}", board_map};
            panic!("{:?} move {:?} to {:?} isn't seen as a valid move", piece, best_from, best_to);
        }
    });
}

fn gen_move_data() -> Result<Vec<(BoardMap, (Position, Position))>> {
    let path = format!("{}/tests/datasets/best_move.xls", env!("CARGO_MANIFEST_DIR"));
    let mut workbook: Xls<_> = open_workbook(path)?;
    let range = workbook.worksheet_range("best_move")
        .ok_or(Error::msg("Cannot find first sheet"))??;

    let iter = RangeDeserializerBuilder::new().from_range(&range)?;
    Ok(iter
        .map(|e| {
            let (fen, best): (String, String) = e.unwrap();
            let (from, to) = (&best[0..2], &best[2..4]);
            let (from, to) = (to_position(from).unwrap(), to_position(to).unwrap());

            assert!(from[0] < 8);
            assert!(from[1] < 8);
            assert!(to[0] < 8);
            assert!(to[1] < 8);

            (
                BoardMap::from_fen(fen),
                (from, to)
            )
        })
        .collect::<Vec<_>>())
}


fn to_position(piece_move: &str) -> Result<Position> {
    let mut piece_move = piece_move.chars().collect::<Vec<_>>();
    piece_move.reverse();

    let rank = (piece_move
        .pop()
        .ok_or(anyhow!("can't parse"))? as usize)
        .sub(97);

    let file = piece_move
        .pop()
        .ok_or(anyhow!("can't parse"))?
        .to_string()
        .parse::<usize>()?
        .sub(1);

    Ok([file, rank])
}