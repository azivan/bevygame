use bevy::prelude::*;

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
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .run();


}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite::from_image(asset_server.load("test.png")));
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
