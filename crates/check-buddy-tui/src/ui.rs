use crate::app::App;
use check_buddy_core::piece_color::PieceColor;
use check_buddy_core::piece_type::PieceType;
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
            let piece_color = match piece.get_color() {
                PieceColor::Black => 'B',
                PieceColor::White => 'W',
            };

            let piece_type = match piece.get_type().unwrap() {
                PieceType::Rook => 'R',
                PieceType::Pawn(_) => 'P',
                PieceType::King => 'K',
                PieceType::Queen => 'Q',
                PieceType::Bishop => 'B',
                PieceType::Knight => 'N',
            };

            let mut text = String::with_capacity(2);
            text.push(piece_color);
            text.push(piece_type);

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
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(chunks[1]);

    let titles = app
        .tab_titles
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

    let uci_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    let uci_input = Paragraph::new(app.input.as_ref()).block(uci_block);
    f.render_widget(uci_input, bottom_layout[1]);
    f.set_cursor(
        // Put cursor past the end of the input text
        bottom_layout[1].x + app.input.len() as u16 + 1,
        // Move one line down, from the border to the input line
        bottom_layout[1].y + 1,
    )
}
