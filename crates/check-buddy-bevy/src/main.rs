use bevy::prelude::*;
use check_buddy_bevy::board_plugin::BoardPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Chess".to_string(),
        width: 400.,
        height: 400.,
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(BoardPlugin);
    app.add_startup_system(setup_camera);
    app.run();
}

#[cfg(not(feature = "debug"))]
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
