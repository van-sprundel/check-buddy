use crate::board_plugin::{BoardMap, PieceComponent};
use crate::events::{PieceClickedEvent, PieceReleasedEvent};
use crate::resources::board_options::BoardOptions;
use bevy::prelude::*;
use clap::command;

pub fn move_system(
    board_options: Res<BoardOptions>,
    mut commands: Commands,
    mut board_map: ResMut<BoardMap>,
    mut piece_clicked_evr: EventReader<PieceClickedEvent>,
    mut piece_released_evr: EventReader<PieceReleasedEvent>,
) {
    for ev in piece_clicked_evr.iter() {
        board_map.selected_square = Some(ev.0);
    }

    for ev in piece_released_evr.iter() {
        let new_pos = ev.0;
        if let Some(old_pos) = board_map.selected_square {
            info!("{:?} {:?}", old_pos, new_pos);
            if let Some(old_piece) = board_map.pieces.clone().get(&old_pos) {
                if board_map.board.move_turn(old_pos, new_pos) {
                    let transform = Transform::from_xyz(
                        (-board_options.tile_size * (7 - new_pos[1]) as f32)
                            - (board_options.tile_size / 2.),
                        (-board_options.tile_size * (7 - new_pos[0]) as f32)
                            - (board_options.tile_size / 2.),
                        1.,
                    );
                    commands
                        .entity(*old_piece)
                        .remove::<Transform>()
                        .insert(transform);

                    if let Some(piece_to_replace) = board_map.pieces.get(&new_pos) {
                        commands.entity(*piece_to_replace).despawn();
                    }

                    board_map.selected_square = None;

                    if let Some(p) = board_map.pieces.remove(&old_pos) {
                        board_map.pieces.insert(new_pos, p);
                    }
                }
            }
        }
        info!("\n{:?}", board_map.board);
    }
}
