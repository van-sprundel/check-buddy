use crate::states::PointerState;
use check_buddy::position_move::{Position, PositionMove};
use check_buddy::{BoardMap, Piece};
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct Board {
    pub(crate) board_map: BoardMap,
    pub(crate) board_conf: BoardConf,
    pub(crate) position_size: f32,
    pub(crate) position_padding: f32,
    pointer_state: PointerState,
    selected_piece: Option<Piece>,
    selected_piece_position: Option<Position>,
    selected_piece_move_positions: Option<Vec<Position>>,
}

#[derive(Default)]
pub struct BoardConf {
    textures: HashMap<String, Texture2D>,
    pub(crate) dark_square: Color,
    pub(crate) light_square: Color,
}

impl Board {
    pub async fn draw_board(&mut self) {
        for y in 0..8 {
            for x in 0..8 {
                let piece = self.board_map.get_piece([y, x]);
                self.draw_board_square(x, y);
                if piece.is_piece() {
                    let (x_pos, y_pos) = (
                        (x as f32 * self.position_size) - self.position_padding,
                        y as f32 * self.position_size,
                    );
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
        if possible_piece.is_piece()
            && possible_piece.get_color() == *self.board_map.get_active_color()
        {
            self.selected_piece = Some(possible_piece);
            self.selected_piece_position = Some([y, x]);
            self.selected_piece_move_positions = Some(self.board_map.gen_legal_positions([y, x]));

            self.pointer_state = PointerState::PieceSelected;
        }
    }

    async fn handle_piece_selected_state(&mut self) {
        if is_mouse_button_released(MouseButton::Left) {
            // get cursor position
            let (x_pos, y_pos) = mouse_position();
            let (x, y) = (
                (x_pos / self.position_size) as usize,
                (y_pos / self.position_size) as usize,
            );

            if let Some(positions) = self.selected_piece_move_positions.clone() {
                if positions.contains(&[y, x]) {
                    let piece_move =
                        PositionMove::new(self.selected_piece_position.unwrap(), [y, x]);
                    match self.board_map.single_move_turn(piece_move) {
                        Ok(_) => {}
                        Err(e) => println!("Invalid move! ({})", e),
                    }
                }
            }

            self.pointer_state = PointerState::Default;
            return;
        }

        assert!(self.selected_piece.is_some());

        // draw square over selected piece
        let position = self.selected_piece_position.unwrap();
        self.draw_board_square(position[1], position[0]);

        // draw moves
        if let Some(positions) = self.selected_piece_move_positions.clone() {
            for position in positions {
                let (mut y, mut x) = (
                    position[0] as f32 * self.position_size,
                    position[1] as f32 * self.position_size,
                );
                if self.board_map.get_piece(position).is_piece() {
                    draw_rectangle(
                        x,
                        y,
                        self.position_size,
                        self.position_size,
                        Color::from_rgba(255, 0, 0, 50),
                    );
                } else {
                    x += self.position_size / 2.;
                    y += self.position_size / 2.;
                    draw_circle(
                        x,
                        y,
                        self.position_size.sqrt(),
                        Color::from_rgba(0, 0, 0, 25),
                    );
                }
            }
        }

        // draw piece to cursor
        let (mut x_pos, mut y_pos) = mouse_position();
        x_pos -= self.position_size / 2.;
        y_pos -= self.position_size / 2.;
        let piece = self.selected_piece.unwrap();
        self.draw_piece(piece, x_pos, y_pos, self.position_size)
            .await;
    }

    fn draw_board_square(&mut self, x: usize, y: usize) {
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
                self.board_conf.light_square
            } else {
                self.board_conf.dark_square
            },
        );
    }

    async fn draw_piece(&mut self, piece: Piece, x_pos: f32, y_pos: f32, size: f32) {
        if !self
            .board_conf
            .textures
            .contains_key(piece.get_icon().unwrap())
        {
            let path = &*("../../assets/".to_owned() + piece.get_icon().unwrap());
            self.board_conf.textures.insert(
                piece.get_icon().unwrap().to_string(),
                load_texture(path).await.unwrap(),
            );
        }

        let texture = self
            .board_conf
            .textures
            .get(piece.get_icon().unwrap())
            .unwrap();
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
