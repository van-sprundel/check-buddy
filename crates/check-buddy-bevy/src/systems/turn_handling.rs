use crate::board_options::BoardOptions;
use bevy::prelude::*;
use check_buddy_core::{ChessEngine, PieceColor};

use crate::board_plugin::Board;
use crate::events::OpponentTurnEvent;

pub fn turn_handle_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    board_options: Res<BoardOptions>,
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut engine: ResMut<ChessEngine>,
    mut opponent_turn_evr: EventReader<OpponentTurnEvent>,
) {
    for _ev in opponent_turn_evr.iter() {
        let best_move = engine.find_best_move_minimax_ab(board.board_map, 3);
        if board.pieces.get(&best_move.from).is_none() {
            panic!("{:?} doesn't contain a piece!", best_move.from);
        }
        if let Some(old_piece) = board.pieces.clone().get(&best_move.from) {
            let is_hit = board.board_map.is_hit(best_move.to);

            board
                .board_map
                .move_turn(best_move)
                .expect("Engine returned invalid move.");

            let transform = Transform::from_xyz(
                (-board_options.tile_size * (7 - best_move.to[1]) as f32)
                    - (board_options.tile_size / 2.),
                (-board_options.tile_size * (7 - best_move.to[0]) as f32)
                    - (board_options.tile_size / 2.),
                1.,
            );
            if best_move.en_passant {
                let shift = match board.board_map.get_piece(best_move.from).get_color() {
                    PieceColor::Black => -1,
                    PieceColor::White => 1,
                };

                let step_new_pos = [
                    ((best_move.to[0] as isize) - shift) as usize,
                    best_move.to[1],
                ];
                if let Some(step_new_piece) = board.pieces.get(&step_new_pos) {
                    commands.entity(*step_new_piece).despawn();
                }
            }

            commands
                .entity(*old_piece)
                .remove::<Transform>()
                .insert(transform);

            if let Some(piece_to_replace) = board.pieces.get(&best_move.to) {
                commands.entity(*piece_to_replace).despawn();
            }

            board.selected_square = None;

            if let Some(p) = board.pieces.remove(&best_move.from) {
                board.pieces.insert(best_move.to, p);
            }

            let move_sound = asset_server.load("se/move_sound.wav");
            let hit_sound = asset_server.load("se/hit_sound.wav");
            audio.play_with_settings(
                if !is_hit { move_sound } else { hit_sound },
                PlaybackSettings {
                    volume: 0.5,
                    ..default()
                },
            );
        }
    }
}
