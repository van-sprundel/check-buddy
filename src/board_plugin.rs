use bevy::app::App;
use bevy::prelude::*;
use std::collections::HashMap;

use crate::events::{PieceClickedEvent, PieceReleasedEvent};
use crate::resources::board::BoardMap;
use crate::resources::board_options::BoardOptions;
use crate::resources::piece::Position;

use crate::systems;
use crate::systems::moves_display::{
    hide_moves, hide_piece_to_cursor, piece_to_cursor, show_moves, spawn_piece_to_cursor, MoveState,
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoardOptions::default())
            .insert_resource(Board::default())
            .add_event::<PieceClickedEvent>()
            .add_event::<PieceReleasedEvent>()
            .add_state(MoveState::Released)
            .add_system_set(
                SystemSet::on_enter(MoveState::Clicked).with_system(spawn_piece_to_cursor),
            )
            .add_system_set(SystemSet::on_update(MoveState::Clicked).with_system(piece_to_cursor))
            .add_system_set(
                SystemSet::on_exit(MoveState::Clicked).with_system(hide_piece_to_cursor),
            )
            .add_startup_system(spawn_board)
            .add_system(show_moves)
            .add_system(hide_moves)
            .add_system(systems::input_handling::input_handle_system)
            .add_system(systems::move_handling::move_system);
    }
}

pub struct Board {
    pub(crate) board_map: BoardMap,
    pub pieces: HashMap<Position, Entity>,
    pub(crate) selected_square: Option<Position>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board_map: BoardMap::from_fen(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ),
            pieces: HashMap::with_capacity(64),
            selected_square: None,
        }
    }
}

impl Board {
    pub fn get_position(
        &self,
        board_options: &Res<BoardOptions>,
        position: Vec2,
    ) -> Option<Position> {
        let board_pos = position / board_options.tile_size;
        if board_pos.x >= 8. || board_pos.x < 0. || board_pos.y >= 8. || board_pos.y < 0. {
            return None;
        }
        Some([board_pos.y as usize, board_pos.x as usize])
    }
}

pub fn spawn_board(
    board_options: Res<BoardOptions>,
    mut board_map: ResMut<Board>,
    windows: Res<Windows>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = windows.primary();
    commands
        .spawn()
        .insert(Name::new("Board"))
        .insert(Transform::from_xyz(
            window.width() / 2.,
            window.height() / 2.,
            0.,
        ))
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            (0..64).for_each(|i| {
                let (x, y) = (i / 8, i % 8);
                let pos = Transform::from_xyz(
                    (-board_options.tile_size * x as f32) - (board_options.tile_size / 2.),
                    (-board_options.tile_size * y as f32) - (board_options.tile_size / 2.),
                    0.,
                );
                // tile
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: if (x + y) % 2 == 0 {
                            board_options.tile_color_white
                        } else {
                            board_options.tile_color_black
                        },
                        custom_size: Some(Vec2::splat(board_options.tile_size)),
                        ..Default::default()
                    },
                    transform: pos,
                    ..Default::default()
                });
                // piece
                let position = [7 - y, 7 - x];
                let piece = board_map.board_map.get_piece(position);
                if let Some(texture) = piece.get_icon(&asset_server) {
                    let entity = parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(board_options.tile_size)),
                                ..Default::default()
                            },
                            texture,
                            transform: pos,
                            ..Default::default()
                        })
                        .insert(PieceComponent(position))
                        .id();
                    board_map.pieces.insert(position, entity);
                }
            });
        });
}

#[derive(Component)]
pub struct PieceComponent(Position);
