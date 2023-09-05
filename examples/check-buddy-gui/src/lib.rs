pub(crate) mod board;
pub(crate) mod states;

use crate::board::Board;
use check_buddy::BoardMap;
use macroquad::prelude::*;

pub async fn run() {
    let mut board = Board::default();
    board.board_map = BoardMap::starting();
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
