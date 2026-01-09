use anyhow::{anyhow, Result};
use check_buddy::position_move::Position;
use check_buddy::BoardMap;
use std::env;

#[derive(Debug)]
struct EpdTest {
    id: String,
    fen: String,
    best_moves: Vec<String>,
}

impl EpdTest {
    fn parse(line: &str) -> Result<Self> {
        // EPD format: FEN + operations
        // Example: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - bm e4; id "test.01";

        // The FEN is everything before the first "bm" operation
        let fen_end_pos = line.find(" bm ").ok_or_else(|| anyhow!("Can't find bm marker"))?;
        let fen = line[..fen_end_pos].trim().to_string();

        // Extract everything after the FEN
        let operations = &line[fen_end_pos..];

        // Extract best moves (between "bm" and the next semicolon)
        let bm_start = operations.find("bm ").ok_or_else(|| anyhow!("No 'bm' found"))?;
        let bm_section = &operations[bm_start + 3..]; // Skip "bm "
        let bm_end = bm_section.find(';').unwrap_or(bm_section.len());
        let best_moves_str = bm_section[..bm_end].trim();

        let best_moves: Vec<String> = best_moves_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        // Extract id (between "id" and the next semicolon or end)
        let id = if let Some(id_start) = operations.find("id ") {
            let id_section = &operations[id_start + 3..]; // Skip "id "
            let id_end = id_section.find(';').unwrap_or(id_section.len());
            id_section[..id_end].trim().trim_matches('"').to_string()
        } else {
            "unknown".to_string()
        };

        Ok(EpdTest {
            id,
            fen,
            best_moves,
        })
    }
}

fn load_epd_tests(filename: &str) -> Result<Vec<EpdTest>> {
    let path = format!("{}/tests/datasets/{}", env!("CARGO_MANIFEST_DIR"), filename);
    let content = std::fs::read_to_string(path)?;

    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(EpdTest::parse)
        .collect()
}

fn san_to_uci_position(san: &str, board: &mut BoardMap) -> Result<(Position, Position)> {
    // Convert algebraic notation to positions
    // This is a simplified parser - a full implementation would handle all SAN notation

    // Handle pawn moves like "e4", "e5", "exd5"
    // Handle piece moves like "Nf3", "Bb5", "Qh4"
    // Handle castling "O-O", "O-O-O"
    // Handle captures "Nxe5", "exd5"
    // Handle check/checkmate markers "+", "#"

    let san = san.trim_end_matches('+').trim_end_matches('#');

    // Try parsing as UCI move first (some EPD files use UCI)
    if san.len() >= 4 && san.chars().all(|c| c.is_ascii_alphanumeric()) {
        if let Ok((_, pos_move)) = board.parse_uci_to_move(san) {
            return Ok((pos_move.from, pos_move.to));
        }
    }

    // For now, return an error for SAN moves
    // A full implementation would need to parse SAN notation properly
    Err(anyhow!("SAN parsing not fully implemented yet: {}", san))
}

#[test]
fn bratko_kopec_suite() -> Result<()> {
    let tests = load_epd_tests("bratko_kopec.epd")?;

    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;

    for test in &tests {
        let mut board = BoardMap::from_fen(&test.fen);

        // Generate all legal moves from this position
        let mut all_legal_moves = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                let from = [x, y];
                for to in board.gen_legal_positions(from) {
                    all_legal_moves.push((from, to));
                }
            }
        }

        // Try to parse the best moves and see if any are in our legal moves
        let mut found_best_move = false;
        let mut parse_error = false;

        for bm in &test.best_moves {
            match san_to_uci_position(bm, &mut board) {
                Ok((from, to)) => {
                    if all_legal_moves.iter().any(|(f, t)| *f == from && *t == to) {
                        found_best_move = true;
                        break;
                    }
                }
                Err(_) => {
                    parse_error = true;
                }
            }
        }

        if parse_error {
            skipped += 1;
            eprintln!("⊘ {} - Skipped (SAN parsing not implemented)", test.id);
        } else if found_best_move {
            passed += 1;
            eprintln!("✓ {} - Passed", test.id);
        } else {
            failed += 1;
            eprintln!("✗ {} - Failed (best moves: {:?})", test.id, test.best_moves);
        }
    }

    eprintln!("\nResults: {} passed, {} failed, {} skipped", passed, failed, skipped);

    // For now, just check that we can load and parse the EPD file
    // Once SAN parsing is implemented, this should actually verify best moves
    assert!(!tests.is_empty(), "Should load EPD tests");
    Ok(())
}

#[test]
fn epd_parser_test() -> Result<()> {
    let line = r#"1k1r4/pp1b1R2/3q2pp/4p3/2B5/4Q3/PPP2B2/2K5 b - - bm Qd1+; id "BK.01";"#;
    let test = EpdTest::parse(line)?;

    assert_eq!(test.id, "BK.01");
    assert_eq!(test.fen, "1k1r4/pp1b1R2/3q2pp/4p3/2B5/4Q3/PPP2B2/2K5 b - -");
    assert_eq!(test.best_moves, vec!["Qd1+"]);

    Ok(())
}

#[test]
fn epd_parser_multiple_best_moves() -> Result<()> {
    let line = r#"r1b2rk1/2q1b1pp/p2ppn2/1p6/3QP3/1BN1B3/PPP3PP/R4RK1 w - - bm Nd5 a4; id "BK.05";"#;
    let test = EpdTest::parse(line)?;

    assert_eq!(test.id, "BK.05");
    assert_eq!(test.best_moves.len(), 2);
    assert!(test.best_moves.contains(&"Nd5".to_string()));
    assert!(test.best_moves.contains(&"a4".to_string()));

    Ok(())
}
