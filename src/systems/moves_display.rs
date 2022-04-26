use crate::board_plugin::BoardMap;
use crate::events::{PieceClickedEvent, PieceReleasedEvent};
use crate::resources::board_options::BoardOptions;
use bevy::prelude::*;
use clap::command;

pub fn show_moves(
     board_map: Res<BoardMap>,
    board_options: Res<BoardOptions>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut piece_clicked_evr: EventReader<PieceClickedEvent>,
) {
    let window = windows.primary();
    for e in piece_clicked_evr.iter() {
        info!("Showing moves");
        commands
            .spawn()
            .insert(Name::new("Move overlay"))
            .insert(Transform::from_xyz(
                window.width() / 2.,
                window.height() / 2.,
                2.,
            ))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                let moves = board_map.board.gen_legal_moves(e.0);
                for position in moves.iter() {
                    parent.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(board_options.tile_size)),
                            color: board_options.move_color,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            -(board_options.tile_size * 8.)
                                + (board_options.tile_size * position[1] as f32)
                                + (board_options.tile_size / 2.),
                            -(board_options.tile_size * 8.)
                                + (board_options.tile_size * position[0] as f32)
                                + (board_options.tile_size / 2.),
                            2.,
                        ),
                        ..Default::default()
                    });
                }
            })
            .insert(MoveOverlay);
    }
}

#[derive(Debug, Clone, Component)]
pub struct MoveOverlay;

pub fn hide_moves(
    mut commands: Commands,
    mut piece_released_evr: EventReader<PieceReleasedEvent>,
    query: Query<Entity, With<MoveOverlay>>,
) {
    for _ev in piece_released_evr.iter() {
        info!("Hiding moves");
        for e in query.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}
