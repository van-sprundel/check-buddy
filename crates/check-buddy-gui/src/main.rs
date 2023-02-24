use macroquad::math::f32;
use check_buddy_core::{BoardMap, Piece};
use macroquad::miniquad::conf::Icon;
use macroquad::miniquad::log;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Checkbuddy".to_owned(),
        window_height: 400,
        window_width: 400,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board {
        board_map: BoardMap::starting(),
        ..Default::default()
    };
    loop {
        clear_background(BLACK);

        board.position_size = screen_height() / 8.;
        board.position_padding = (screen_height() - screen_width()) / 2.;

        draw_board(&board).await;

        match board.pointer_state {
            PointerState::Default => handle_default_state(&mut board),
            PointerState::PieceSelected => handle_piece_selected_state(&mut board).await,
        }

        next_frame().await
    }
}

#[derive(Default)]
struct Board {
    position_size: f32,
    position_padding: f32,
    board_map: BoardMap,
    pointer_state: PointerState,
    selected_piece: Option<Piece>,
}

async fn handle_piece_selected_state(board: &mut Board) {
    if is_mouse_button_released(MouseButton::Left) {
        board.pointer_state = PointerState::Default;
        return;
    }
    assert!(board.selected_piece.is_some());

    let (mut x_pos, mut y_pos) = mouse_position();
    x_pos -= board.position_size / 2.;
    y_pos -= board.position_size / 2.;
    let piece = board.selected_piece.unwrap();
    draw_piece(x_pos, y_pos, board.position_size, piece.get_icon().expect("Couldn't get icon")).await;
}

async fn draw_board(board: &Board) {
    for y in 0..8 {
        for x in 0..8 {
            let position = board.board_map.get_piece([y, x]);
            let dark_square = Color::from_rgba(118, 150, 86, 255);
            let light_square = Color::from_rgba(238, 238, 210, 255);
            let (x_pos, y_pos) = (
                (x as f32 * board.position_size) - board.position_padding,
                y as f32 * board.position_size,
            );
            draw_rectangle(
                x_pos,
                y_pos,
                board.position_size,
                board.position_size,
                if (x + y) % 2 == 0 {
                    light_square
                } else {
                    dark_square
                },
            );
            if let Some(texture) = position.get_icon() {
                draw_piece(x_pos, y_pos, board.position_size, texture).await;
            }
        }
    }
}

fn handle_default_state(board: &mut Board) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mut x_pos, y_pos) = mouse_position();
        x_pos += board.position_padding;

        let (x, y) = ((x_pos / 50.) as usize, (y_pos / 50.) as usize);
        if x >= 8 || y >= 8 {
            return;
        }
        let possible_piece = board.board_map.get_piece([y, x]);
        if possible_piece.is_piece() {
            board.selected_piece = Some(possible_piece);
            board.pointer_state = PointerState::PieceSelected;
        }
    }
}

async fn draw_piece(x_pos: f32, y_pos: f32, size: f32, texture: &str) {
    draw_texture_ex(
        load_texture(&*("crates/check-buddy-gui/assets/".to_owned() + texture))
            .await
            .unwrap(),
        x_pos,
        y_pos,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(size, size)),
            ..Default::default()
        },
    )
}

#[derive(Default)]
enum PointerState {
    #[default]
    Default,
    PieceSelected,
}
