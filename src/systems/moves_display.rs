use crate::board_plugin::Board;
use crate::events::{PieceClickedEvent, PieceReleasedEvent};
use crate::resources::board_options::BoardOptions;
use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MoveState {
    Clicked,
    Released,
}

pub fn show_moves(
    board_map: Res<Board>,
    board_options: Res<BoardOptions>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut piece_clicked_evr: EventReader<PieceClickedEvent>,
    mut state: ResMut<State<MoveState>>,
) {
    let window = windows.primary();
    for ev in piece_clicked_evr.iter() {
        if *state.current() == MoveState::Released {
            state.set(MoveState::Clicked).unwrap();
        }
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
                let moves = board_map.board_map.gen_legal_moves(ev.0);
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
        commands
            .spawn()
            .insert(Transform::from_xyz(
                window.width() / 2.,
                window.height() / 2.,
                2.,
            ))
            .insert(GlobalTransform::default())
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(board_options.tile_size)),
                    color: if (ev.0[0] + ev.0[1]) % 2 != 0 {
                        board_options.tile_color_black
                    } else {
                        board_options.tile_color_white
                    },
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    -200.
                        + (board_options.tile_size * (ev.0[1] as f32))
                        + (board_options.tile_size / 2.),
                    -200.
                        + (board_options.tile_size * (ev.0[0] as f32))
                        + (board_options.tile_size / 2.),
                    2.,
                ),
                ..Default::default()
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
    mut state: ResMut<State<MoveState>>,
) {
    for _ev in piece_released_evr.iter() {
        if *state.current() == MoveState::Clicked {
            state.set(MoveState::Released).unwrap();
        }
        info!("Hiding moves");
        for e in query.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}

pub fn spawn_piece_to_cursor(
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    board_options: Res<BoardOptions>,
    board_map: Res<Board>,
    mut commands: Commands,
) {
    let window = windows.primary_mut();
    window.set_cursor_visibility(false);
    if let Some(selected_position) = board_map.selected_square {
        let piece = board_map.board_map.get_piece(selected_position);

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(board_options.tile_size)),
                    ..Default::default()
                },
                texture: piece.get_icon(&asset_server).unwrap(),
                transform: Transform::from_xyz(
                    selected_position[0] as f32 * board_options.tile_size,
                    selected_position[1] as f32 * board_options.tile_size,
                    5.,
                ),
                ..Default::default()
            })
            .insert(CursorPiece);
    }
}

#[derive(Component)]
pub struct CursorPiece;

pub fn piece_to_cursor(
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<CursorPiece>>,
) {
    let window = windows.primary();
    for mut cursor_piece in query.iter_mut() {
        if let Some(position) = window.cursor_position() {
            // if let Some(position2) = board_map.get_position(&board_options, position) {
            cursor_piece.translation = Vec3::new(
                -(window.width() / 2.) + position[0] as f32,
                -(window.height() / 2.) + position[1] as f32,
                5.,
            );
            // }
        }
    }
}

pub fn hide_piece_to_cursor(
    mut windows: ResMut<Windows>,
    query: Query<Entity, With<CursorPiece>>,
    mut commands: Commands,
) {
    let window = windows.primary_mut();
    for cursor_piece in query.iter() {
        commands.entity(cursor_piece).despawn();
        window.set_cursor_visibility(true);
    }
}
