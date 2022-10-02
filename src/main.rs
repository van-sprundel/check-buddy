use bevy::app::App;
use bevy::DefaultPlugins;
use check_buddy::board_plugin::BoardPlugin;

#[cfg(feature = "debug")]
use colored::*;
#[cfg(feature = "debug")]
use std::io;

#[cfg(not(feature = "debug"))]
use bevy::prelude::*;
#[cfg(feature = "debug")]
use check_buddy::resources::board::BoardMap;

#[cfg(feature = "debug")]
fn main() {
    let mut board = BoardMap::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    loop {
        eprintln!("{:?}", board);

        let mut buffer = String::new();
        let mut stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();

        buffer.retain(|c| !c.is_whitespace());
        if buffer.len() < 4 {
            eprintln!("{}", "Invalid syntax".red());
            continue;
        }

        // TODO make function to test which of the 2 possibilites can actually make the legal move
        let sections = buffer.chars().collect::<Vec<_>>();
        // let piece_to_move = sections[0];
        // let capture = sections[1] == 'x';
        // let start = if capture { 2 } else { 1 };
        // let move_to = sections[start..start + 2];
        let from_pos = &sections[0..2];
        let from_file = match from_pos[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => {
                eprintln!("{}", "Invalid syntax".red());
                continue;
            }
        };
        let from_rank = *&from_pos[1].to_digit(10).unwrap() as usize;
        let from = [from_rank - 1, from_file];

        if !(0..=8).contains(&to_file)
            || !(0..=8).contains(&from_file)
            || !(0..=8).contains(&to_rank)
            || !(0..=8).contains(&from_rank)
        {
            eprintln!("Move out of bounds");
            continue;
        }

        board.move_turn((from, to));
    }
}

#[cfg(not(feature = "debug"))]
fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Chess".to_string(),
        width: 400.,
        height: 400.,
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    // app.add_plugin(WorldInspectorPlugin::new());
    app.add_plugin(BoardPlugin);
    app.add_startup_system(setup_camera);
    app.run();
}

#[cfg(not(feature = "debug"))]
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
