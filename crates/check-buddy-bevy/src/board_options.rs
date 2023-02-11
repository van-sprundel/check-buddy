use bevy::prelude::*;

#[derive(Resource)]
pub struct BoardOptions {
    pub tile_color_white: Color,
    pub tile_color_black: Color,
    pub move_color: Color,
    pub tile_size: f32,
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            tile_color_white: Color::hex("F0D9B5").unwrap(),
            tile_color_black: Color::hex("B58863").unwrap(),
            move_color: Color::rgba(1., 0., 0., 0.5),
            tile_size: 50.,
        }
    }
}
