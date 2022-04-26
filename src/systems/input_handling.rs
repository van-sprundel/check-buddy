use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use bevy::prelude::*;

use crate::board_plugin::BoardMap;
use crate::events::{PieceClickedEvent, PieceReleasedEvent};
use crate::resources::board::Board;
use crate::resources::board_options::BoardOptions;

pub fn input_handle_system(
    board_options: Res<BoardOptions>,
    windows: Res<Windows>,
    board_map: Res<BoardMap>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut piece_clicked_wr: EventWriter<PieceClickedEvent>,
    mut piece_released_wr: EventWriter<PieceReleasedEvent>,
) {
    let window = windows.primary();
    for e in button_evr.iter() {
        if let ElementState::Pressed = e.state {
            let cursor_position = window.cursor_position().unwrap();
            if let Some(position) = board_map.get_position(&board_options, cursor_position) {
                let piece = board_map.board.get_piece(position);

                if board_map.board.get_active_color() != piece.get_color() {
                    return;
                }
                if e.button == MouseButton::Left {
                    info!("Clicking on tile {:?}", position);
                    if piece.is_piece() {
                        piece_clicked_wr.send(PieceClickedEvent(position));
                    }
                }
            }
        }
        if let ElementState::Released = e.state {
            let cursor_position = window.cursor_position().unwrap();
            if let Some(position) = board_map.get_position(&board_options, cursor_position) {
                if e.button == MouseButton::Left {
                    piece_released_wr.send(PieceReleasedEvent(position));
                }
            }
        }
    }
}
