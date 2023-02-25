use crate::app::App;
use check_buddy_core::piece_type::PieceType;
use check_buddy_core::PieceColor;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Tabs};
use tui::Frame;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(40), Constraint::Min(8)].as_ref())
        .margin(1)
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::White));
    f.render_widget(block, size);
    let main_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(72), Constraint::Min(8)].as_ref())
        .split(chunks[0]);

    let checker_constraints = [Constraint::Ratio(1, 8); 8].as_ref();
    let board_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(checker_constraints)
        .split(main_chunk[0]);
    let checker = Layout::default().constraints(checker_constraints);
    let row0 = checker.split(board_layout[0]);
    let row1 = checker.split(board_layout[1]);
    let row2 = checker.split(board_layout[2]);
    let row3 = checker.split(board_layout[3]);
    let row4 = checker.split(board_layout[4]);
    let row5 = checker.split(board_layout[5]);
    let row6 = checker.split(board_layout[6]);
    let row7 = checker.split(board_layout[7]);

    let rows = [row0, row1, row2, row3, row4, row5, row6, row7];
    for i in 0..64 {
        let (row, col) = (i / 8, i % 8);

        let block = Block::default()
            .style(Style::default().fg(if (row + col) % 2 == 0 {
                Color::White
            } else {
                Color::DarkGray
            }))
            .borders(Borders::ALL);

        let piece = app.board_map.get_piece([col, row]);

        if piece.is_piece() {
            let text = match (piece.get_color().unwrap(), piece.get_type().unwrap()) {
                (PieceColor::Black, PieceType::Pawn(_)) => "BP",
                (PieceColor::Black, PieceType::Rook) => "BR",
                (PieceColor::Black, PieceType::Knight) => "BN",
                (PieceColor::Black, PieceType::Bishop) => "BB",
                (PieceColor::Black, PieceType::King) => "BK",
                (PieceColor::Black, PieceType::Queen) => "BQ",
                (PieceColor::White, PieceType::Pawn(_)) => "WP",
                (PieceColor::White, PieceType::Rook) => "WR",
                (PieceColor::White, PieceType::Knight) => "WN",
                (PieceColor::White, PieceType::Bishop) => "WB",
                (PieceColor::White, PieceType::King) => "WK",
                (PieceColor::White, PieceType::Queen) => "WQ",
            };
            let paragraph = Paragraph::new(text)
                .alignment(Alignment::Center)
                .block(block);
            f.render_widget(paragraph, rows[row][col]);
        } else {
            f.render_widget(block, rows[row][col]);
        }
    }

    let move_index = Block::default().title("Moves").borders(Borders::ALL);
    f.render_widget(move_index, main_chunk[1]);

    let bottom_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Max(4)])
        .split(chunks[1]);

    let titles = app
        .titles
        .iter()
        .enumerate()
        .map(|(i, t)| {
            if i == app.index {
                Spans::from(vec![Span::styled(*t, Style::default().fg(Color::Cyan))])
            } else {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(first, Style::default().fg(Color::Cyan)),
                    Span::styled(rest, Style::default().fg(Color::White)),
                ])
            }
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, bottom_layout[0]);

    let uci_input = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(uci_input, bottom_layout[1]);
}
