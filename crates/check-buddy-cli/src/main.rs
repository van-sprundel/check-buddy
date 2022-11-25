use anyhow::{anyhow, Result};
use check_buddy_core::board::BoardMap;
use check_buddy_core::piece::piece_move::PieceMove;
use check_buddy_core::piece::piece_type::PieceType;
use colored::Colorize;
use std::fmt::{Display, Error};
use std::io;
use std::ops::Sub;

fn main() {
    let mut board = BoardMap::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut buffer = String::new();

    loop {
        println!("{:?}", board);

        let mut stdin = io::stdin();
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        buffer.retain(|c| !c.is_whitespace());

        if let Ok(piece_move) = parse_move(&mut board, &buffer) {
            println!("{:?}", piece_move);
            board.move_turn(piece_move);
        }
    }
}

fn parse_move(board: &mut BoardMap, buffer: &String) -> Result<PieceMove> {
    let non_pawn_move = buffer
        .chars()
        .nth(0)
        .ok_or(anyhow!("can't parse"))?
        .is_uppercase();
    let mut buffer_index = if non_pawn_move { 0 } else { 1 };
    let mut from = None;
    let mut to = None;
    let mut move_data = buffer.chars().collect::<Vec<_>>();
    move_data.reverse();

    let piece_type = match move_data.pop().ok_or(anyhow!("can't parse"))? {
        'B' => PieceType::Bishop,
        'N' => PieceType::Knight,
        'K' => PieceType::King,
        'R' => PieceType::Rook,
        'Q' => PieceType::Queen,
        _ => return Err(anyhow!("can't parse")),
    };
    from = board.find_piece(*board.get_active_color(), piece_type);

    if let Some(position) = from {
        let rank = (move_data.pop().ok_or(anyhow!("can't parse"))? as usize).sub(97);
        let file = move_data
            .pop()
            .ok_or(anyhow!("can't parse"))?
            .to_string()
            .parse::<usize>()?
            .sub(1);

        let moves = board.gen_moves(position);

        if moves.contains(&[rank, file]) {
            return Err(anyhow!("can't parse"));
        }

        to = Some([rank, file]);
    } else {
        //pawn
    }

    buffer_index += 1;

    return Ok(PieceMove::new(
        from.expect("couldn't parse"),
        to.expect("couldn't parse"),
    ));
}
