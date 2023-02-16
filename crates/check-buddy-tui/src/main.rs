use anyhow::{anyhow, Result};
use check_buddy_core::piece_move::PieceMove;
use check_buddy_core::piece_type::PieceType;
use check_buddy_core::BoardMap;
use std::io;
use std::io::Write;
use std::ops::Sub;

fn main() {
    let mut board = BoardMap::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut buffer = String::new();
    let mut stdout = io::stdout();

    loop {
        let _ = stdout.lock().write_all(format!("{:?}", board).as_ref());
        let _ = stdout.lock().write_all("> ".as_ref());
        let _ = stdout.flush();

        let stdin = io::stdin();
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        buffer.retain(|c| !c.is_whitespace());

        if let Ok(piece_move) = board.parse_move(&buffer) {
            println!("{:?}", piece_move);
            board.move_turn(piece_move);
        } else {
            println!("no move :(");
        }
    }
}
