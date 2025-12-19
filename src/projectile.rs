use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (shoot_laser, move_lasers).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Projectile;

fn shoot_laser(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_q: Query<&Transform, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok(player_transform) = player_q.single() {
            commands.spawn((
                Sprite::from_image(asset_server.load("projectile.png")),
                Transform::from_xyz(
                    player_transform.translation.x,
                    player_transform.translation.y + 50.0,
                    0.0,
                ),
                Projectile,
            ));
        }
    }
}

fn move_lasers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Projectile>>,
) {
    for (entity, mut transform) in &mut query {
        transform.translation.y += 500.0 * time.delta_secs();
        if transform.translation.y > 1000.0 {
            commands.entity(entity).despawn();
        }
    }
}
