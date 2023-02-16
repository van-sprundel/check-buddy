use bevy::prelude::*;
use check_buddy_bevy::board_plugin::BoardPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Chess".to_string(),
            width: 400.,
            height: 400.,
            ..default()
        },
        ..default()
    }));
    app.add_plugin(BoardPlugin);
    app.add_startup_system(setup_camera);
    app.run();
}

#[cfg(not(feature = "debug"))]
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
