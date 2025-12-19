use crate::GameState;
use bevy::prelude::*;
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (move_enemies, spawn_enemies).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct EnemySpawnTimer(Timer);

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let mut rng = rand::rng();
        let x = rng.random_range(-300.0..300.0);
        commands.spawn((
            Sprite::from_image(asset_server.load("enemy.png")),
            Transform::from_xyz(x, 400.0, 0.0),
            Enemy,
        ));
    }
}

fn move_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Enemy>>,
) {
    for (entity, mut transform) in &mut query {
        transform.translation.y -= 100.0 * time.delta_secs();

        if transform.translation.y < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}
