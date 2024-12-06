// player.rs
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (player_movement, player_look));
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Camera3d,
        Transform::from_xyz(0.0, 2.0, 5.0),
        Player,
    ));
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        player_transform.translation += direction * 5.0 * time.delta_secs();
    }
}

fn player_look(
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    for motion in mouse_motion.read() {
        player_transform.rotate_y(-motion.delta.x * 0.003);
        player_transform.rotate_x(-motion.delta.y * 0.003);
    }
}