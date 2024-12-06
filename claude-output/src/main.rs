use bevy::{prelude::*, render::mesh::{Indices, PrimitiveTopology}};

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
fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player { speed: 5.0 },
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
    ));
}

fn generate_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut chunk = Chunk::new(IVec3::ZERO, 16);
    
    // Generate some test terrain
    for x in 0..16 {
        for z in 0..16 {
            let height = 8;
            for y in 0..16 {
                if let Some(index) = chunk.get_voxel_index(IVec3::new(x as i32, y as i32, z as i32)) {
                    chunk.voxels[index] = if y < height {
                        VoxelType::Dirt
                    } else if y == height {
                        VoxelType::Grass
                    } else {
                        VoxelType::Air
                    };
                }
            }
        }
    }

    let mesh = create_chunk_mesh(&chunk);
    
    commands.spawn((
        chunk,
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}

fn create_chunk_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    
    for x in 0..chunk.size {
        for y in 0..chunk.size {
            for z in 0..chunk.size {
                if let Some(idx) = chunk.get_voxel_index(IVec3::new(x as i32, y as i32, z as i32)) {
                    if chunk.voxels[idx] != VoxelType::Air {
                        add_cube_faces(
                            Vec3::new(x as f32, y as f32, z as f32),
                            &mut vertices,
                            &mut indices,
                            &mut normals,
                            vertices.len() as u32,
                        );
                    }
                }
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

fn add_cube_faces(
    position: Vec3,
    vertices: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    normals: &mut Vec<[f32; 3]>,
    start_index: u32,
) {
    // Front face
    vertices.extend_from_slice(&[
        [position.x, position.y, position.z],
        [position.x + 1.0, position.y, position.z],
        [position.x + 1.0, position.y + 1.0, position.z],
        [position.x, position.y + 1.0, position.z],
    ]);
    
    for _ in 0..4 {
        normals.push([0.0, 0.0, -1.0]);
    }
    
    indices.extend_from_slice(&[
        start_index,
        start_index + 1,
        start_index + 2,
        start_index,
        start_index + 2,
        start_index + 3,
    ]);
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
        .add_systems(Startup, (setup, spawn_player, generate_chunk))
        .add_systems(Update, player_movement)
        .run();
}
