use anyhow::{anyhow, Result};
use check_buddy_core::BoardMap;
use check_buddy_core::piece_move::PieceMove;
use check_buddy_core::piece_type::PieceType;
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

        if let Ok(piece_move) = parse_move(&mut board, &buffer) {
            println!("{:?}", piece_move);
            board.move_turn(piece_move);
        } else {
            println!("no move :(");
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
    let mut move_data = buffer.chars().collect::<Vec<_>>();
    move_data.reverse();

    let piece_type = if non_pawn_move {
         match move_data.pop().ok_or(anyhow!("can't parse"))? {
            'B' => PieceType::Bishop,
            'N' => PieceType::Knight,
            'K' => PieceType::King,
            'R' => PieceType::Rook,
            'Q' => PieceType::Queen,
            _ => return Err(anyhow!("can't parse")),
        }
    } else {
        //TODO check for piece on position and update
        PieceType::Pawn(false)
    };

    let positions = board.find_piece(*board.get_active_color(), piece_type);
    for from_position in positions {
        let mut move_data = move_data.clone();

        let rank = (move_data
            .pop()
            .ok_or(anyhow!("can't parse"))? as usize)
            .sub(97);

        let file = move_data
            .pop()
            .ok_or(anyhow!("can't parse"))?
            .to_string()
            .parse::<usize>()?
            .sub(1);

        let piece_move = PieceMove::new(from_position, [file, rank]);

        if board.is_valid_move(piece_move) {
            return Ok(piece_move);
        }
    }

    return Err(anyhow!("Not a valid move"));
}
