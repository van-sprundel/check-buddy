use crate::states::PointerState;
use check_buddy_core::{BoardMap, Piece};
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Board {
    position_size: f32,
    position_padding: f32,
    pub(crate) board_map: BoardMap,
    pointer_state: PointerState,
    selected_piece: Option<Piece>,
    textures: HashMap<String, Texture2D>,
}

impl Board {
    pub fn set_position_size(&mut self, position_size: f32) {
        self.position_size = position_size;
    }

    pub fn set_position_padding(&mut self, position_padding: f32) {
        self.position_padding = position_padding;
    }

    pub async fn draw_board(&mut self) {
        for y in 0..8 {
            for x in 0..8 {
                let piece = self.board_map.get_piece([y, x]);
                let dark_square = Color::from_rgba(118, 150, 86, 255);
                let light_square = Color::from_rgba(238, 238, 210, 255);
                let (x_pos, y_pos) = (
                    (x as f32 * self.position_size) - self.position_padding,
                    y as f32 * self.position_size,
                );
                draw_rectangle(
                    x_pos,
                    y_pos,
                    self.position_size,
                    self.position_size,
                    if (x + y) % 2 == 0 {
                        light_square
                    } else {
                        dark_square
                    },
                );
                if piece.is_piece() {
                    self.draw_piece(piece, x_pos, y_pos, self.position_size)
                        .await;
                }
            }
        }
    }

    pub async fn handle_pointer_state(&mut self) {
        match self.pointer_state {
            PointerState::Default => self.handle_default_state(),
            PointerState::PieceSelected => self.handle_piece_selected_state().await,
        }
    }

    fn handle_default_state(&mut self) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }
        let (mut x_pos, y_pos) = mouse_position();
        x_pos += self.position_padding;

        let (x, y) = ((x_pos / 50.) as usize, (y_pos / 50.) as usize);
        if x >= 8 || y >= 8 {
            return;
        }
        let possible_piece = self.board_map.get_piece([y, x]);
        if possible_piece.is_piece() {
            self.selected_piece = Some(possible_piece);
            self.pointer_state = PointerState::PieceSelected;
        }
    }

    async fn handle_piece_selected_state(&mut self) {
        if is_mouse_button_released(MouseButton::Left) {
            self.pointer_state = PointerState::Default;
            return;
        }

        assert!(self.selected_piece.is_some());

        let (mut x_pos, mut y_pos) = mouse_position();
        x_pos -= self.position_size / 2.;
        y_pos -= self.position_size / 2.;
        let piece = self.selected_piece.unwrap();
        self.draw_piece(piece, x_pos, y_pos, self.position_size)
            .await;
    }

    async fn draw_piece(&mut self, piece: Piece, x_pos: f32, y_pos: f32, size: f32) {
        if !self.textures.contains_key(piece.get_icon().unwrap()) {
            let path = &*("crates/check-buddy-gui/assets/".to_owned() + piece.get_icon().unwrap());
            self.textures.insert(
                piece.get_icon().unwrap().to_string(),
                load_texture(path).await.unwrap(),
            );
        }

        let texture = self.textures.get(piece.get_icon().unwrap()).unwrap();
        draw_texture_ex(
            *texture,
            x_pos,
            y_pos,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(size, size)),
                ..Default::default()
            },
        )
    }
}
