pub(crate) mod board;
pub(crate) mod states;

use crate::board::Board;
use check_buddy_core::BoardMap;
use macroquad::prelude::*;

pub async fn run() {
    let mut board = Board::default();
    board.board_map = BoardMap::from_fen("r1q1k1r1/pppb1p2/3p4/2b4p/2BNPp2/2PP2PP/PP3PQK/R3R3");
    board.board_map.switch_active_color();
    board.board_conf.dark_square = Color::from_rgba(118, 150, 86, 255);
    board.board_conf.light_square = Color::from_rgba(238, 238, 210, 255);

    loop {
        clear_background(BLACK);

        board.position_size = screen_height() / 8.;
        board.position_padding = (screen_height() - screen_width()) / 2.;

        board.draw_board().await;
        board.handle_pointer_state().await;

        next_frame().await
    }
}
