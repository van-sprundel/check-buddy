use anyhow::{anyhow, Result};
use check_buddy_core::*;
use std::collections::HashMap;

pub struct PgnParser;

impl PgnParser {
    pub fn parse(buffer: String) -> Result<Game> {
        let mut game = Game::default();
        let (info, uci) = buffer
            .split_once::<&str>("\r\n\r\n")
            .ok_or(anyhow!("Can't parse PGN"))?;

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
        let winning = uci_line.pop();

        for moves in uci_line.iter() {
            println!("{moves:?}");
            let (move1, move2) = (moves[1], moves[2]);
            let historical_move1 = game.board_map.parse_uci_to_historical_move(move1)?;
            println!("{:?}", historical_move1);
            game.board_map.move_turn(historical_move1.1)?;
            let historical_move2 = game.board_map.parse_uci_to_historical_move(move2)?;
            println!("{:?}", historical_move2);
            game.board_map.move_turn(historical_move2.1)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_pgn() -> Vec<u8> {
        let mut path = std::env::current_dir().unwrap();
        path.push("assets\\pgns\\example.pgn");

        std::fs::read(path).unwrap()
    }

    #[test]
    fn should_return_happy_flow() {
        let pgn = String::from_utf8(get_example_pgn()).unwrap();
        let e = PgnParser::parse(pgn).unwrap();
    }
}
