pub(crate) mod board;
pub(crate) mod states;

use check_buddy_core::BoardMap;
use macroquad::prelude::*;
use crate::board::Board;

pub async fn run() {
    let mut board = Board::default();
    board.board_map= BoardMap::starting();

    loop {
        clear_background(BLACK);

        board.set_position_size(screen_height() / 8.);
        board.set_position_padding((screen_height() - screen_width()) / 2.);

        board.draw_board().await;
        board.handle_pointer_state().await;

        next_frame().await
    }
}
