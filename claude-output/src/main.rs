use bevy::prelude::*;

// Component definitions
#[derive(Debug, Clone, Copy, PartialEq)]
enum VoxelType {
    Air,
    Dirt,
    Grass,
    Stone,
}

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Chunk {
    position: IVec3,
    size: usize,
    voxels: Vec<VoxelType>,
}

impl Chunk {
    fn new(position: IVec3, size: usize) -> Self {
        Self {
            position,
            size,
            voxels: vec![VoxelType::Air; size * size * size],
        }
    }

    fn get_voxel_index(&self, pos: IVec3) -> Option<usize> {
        if pos.x < 0 || pos.y < 0 || pos.z < 0 || 
           pos.x >= self.size as i32 || 
           pos.y >= self.size as i32 || 
           pos.z >= self.size as i32 {
            return None;
        }
        Some((pos.x + pos.y * self.size as i32 + pos.z * self.size as i32 * self.size as i32) as usize)
    }
}

// Systems
fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn((
        Player { speed: 5.0 },
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}

fn player_movement(
    mut query: Query<(&mut Transform, &Player)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        if keyboard.pressed(KeyCode::W) { direction.z -= 1.0; }
        if keyboard.pressed(KeyCode::S) { direction.z += 1.0; }
        if keyboard.pressed(KeyCode::A) { direction.x -= 1.0; }
        if keyboard.pressed(KeyCode::D) { direction.x += 1.0; }
        if keyboard.pressed(KeyCode::Space) { direction.y += 1.0; }
        if keyboard.pressed(KeyCode::ShiftLeft) { direction.y -= 1.0; }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * player.speed * time.delta_seconds();
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Temporary ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_player))
        .add_systems(Update, player_movement)
        .run();
}
