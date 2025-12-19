use bevy::prelude::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}
#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("player.png")),
        Transform::from_xyz(0.0, -200.0, 0.0),
        Player { speed: 500.0 },
    ));
}

fn move_player(
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, player) in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * player.speed * time.delta_secs();
        }
    }
}
