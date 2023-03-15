use check_buddy_core::BoardMap;
use std::io;
use std::io::Write;

fn main() {
    let mut board = BoardMap::starting();
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

        if let Ok(historical_move) = board.parse_uci_to_historical_move(&buffer) {
            println!("{:?}", historical_move);
            let _ = board.move_turn(historical_move.1);
        } else {
            println!("no move :(");
        }
    }
}
