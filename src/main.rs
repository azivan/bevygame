mod player;

use bevy::prelude::*;
use player::PlayerPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
        .run();
}

fn menu_input(mut next_state: ResMut<NextState<GameState>>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("Switching state to Playing");
        next_state.set(GameState::Playing);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
