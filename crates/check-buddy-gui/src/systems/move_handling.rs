use crate::board_options::BoardOptions;
use crate::board_plugin::Board;
use crate::events::{PieceClickedEvent, PieceReleasedEvent};
use bevy::prelude::*;
use check_buddy_core::piece_move::PieceMove;
use check_buddy_core::PieceColor;

pub fn move_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    board_options: Res<BoardOptions>,
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut piece_clicked_evr: EventReader<PieceClickedEvent>,
    mut piece_released_evr: EventReader<PieceReleasedEvent>,
) {
    for ev in piece_clicked_evr.iter() {
        board.selected_square = Some(ev.0);
    }

    for ev in piece_released_evr.iter() {
        let new_pos = ev.0;
        if let Some(old_pos) = board.selected_square {
            info!("{:?} {:?}", old_pos, new_pos);
            if let Some(old_piece) = board.pieces.clone().get(&old_pos) {
                let is_hit = board.board_map.is_hit(new_pos);
                let piece_move = PieceMove {
                    from: old_pos,
                    to: new_pos,
                    en_passant: false,
                    trade: false,
                };
                let en_passant = board.board_map.is_en_passant(piece_move);
                if board.board_map.move_turn(piece_move) {
                    let transform = Transform::from_xyz(
                        (-board_options.tile_size * (7 - new_pos[1]) as f32)
                            - (board_options.tile_size / 2.),
                        (-board_options.tile_size * (7 - new_pos[0]) as f32)
                            - (board_options.tile_size / 2.),
                        1.,
                    );
                    if en_passant {
                        let shift = match board.board_map.get_piece(old_pos).get_color() {
                            PieceColor::Black => -1,
                            PieceColor::White => 1,
                        };

                        let step_new_pos = [((new_pos[0] as isize) - shift) as usize, new_pos[1]];
                        // eprintln!("move handling removing piece on {:?}", &step_new_pos);
                        if let Some(step_new_piece) = board.pieces.get(&step_new_pos) {
                            commands.entity(*step_new_piece).despawn();
                        }
                    }

                    commands
                        .entity(*old_piece)
                        .remove::<Transform>()
                        .insert(transform);

                    if let Some(piece_to_replace) = board.pieces.get(&new_pos) {
                        commands.entity(*piece_to_replace).despawn();
                    }

                    board.selected_square = None;

                    if let Some(p) = board.pieces.remove(&old_pos) {
                        board.pieces.insert(new_pos, p);
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

        info!("\n{:?}", board.board_map);
    }
}
