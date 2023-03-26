use anyhow::{anyhow, Result};
use check_buddy_core::*;

pub struct PgnParser;

impl PgnParser {
    pub fn parse(buffer: String) -> Result<Game> {
        let mut game = Game::default();

        #[cfg(target_os = "macos")]
        let empty_line = "\n\n";
        #[cfg(target_os = "windows")]
        let empty_line = "\r\n\r\n";

        let (info, uci) = buffer
            .split_once::<&str>(empty_line)
            .ok_or(anyhow!("Can't split info and UCI"))?;

        Self::parse_info(&mut game, info)?;
        Self::parse_uci(&mut game, uci)?;

        Ok(game)
    }

    fn parse_info(game: &mut Game, info: &str) -> Result<()> {
        info.lines()
            .map(|line| {
                let mut chars = line.chars();
                chars.next();
                chars.next_back();
                chars.as_str()
            })
            .for_each(|line| {
                let (title, content) = line.split_once(' ').expect("Couldn't parse info row");
                let content = content.trim_matches('\"');
                game.info.insert(title.to_string(), content.to_string());
            });
        Ok(())
    }

    fn parse_uci(game: &mut Game, uci: &str) -> Result<()> {
        let binding = uci.split_whitespace().collect::<Vec<&str>>();
        let mut uci_line = binding.chunks(3).collect::<Vec<&[&str]>>();
        let _winning = uci_line.pop();

        for moves in uci_line.iter() {
            println!("{moves:?}");
            let (move1, move2) = (moves[1], moves[2]);

            let uci_move1 = game.board_map.parse_pgn_to_uci_move(move1)?;
            println!("{:?}", uci_move1);
            game.board_map.uci_move_turn(uci_move1)?;
            println!("{:?}", game.board_map);
            let uci_move2 = game.board_map.parse_pgn_to_uci_move(move2)?;
            println!("{:?}", uci_move2);
            game.board_map.uci_move_turn(uci_move2)?;
            println!("{:?}", game.board_map);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_pgn() -> Vec<u8> {
        let mut path = std::env::current_dir().unwrap();
        #[cfg(target_os = "macos")]
        path.push("assets/pgns/example.pgn");
        #[cfg(target_os = "windows")]
        path.push("assets\\pgns\\example.pgn");

        std::fs::read(path).unwrap()
    }

    #[test]
    fn should_return_happy_flow() {
        let pgn = String::from_utf8(get_example_pgn()).unwrap();
        let _ = PgnParser::parse(pgn).unwrap();
    }
}
