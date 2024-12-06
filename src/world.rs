// world.rs
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a detailed ground plane
    let plane_size = 1000.0;
    let plane_mesh = Mesh::from(shape::Plane::new(plane_size));

    let ground_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.2, 0.8, 0.2),
        perceptual_roughness: 0.9,
        ..default()
    });

    // Spawn the ground using Mesh3d
    commands.spawn((
        Mesh3d(meshes.add(plane_mesh)),
        MeshMaterial3d(ground_material),
        Transform::default(),
    ));

    // Directional light
    commands.spawn(DirectionalLight {
        illuminance: 10000.0,
        shadows_enabled: true,
        transform: Transform::from_xyz(50.0, 100.0, 50.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}