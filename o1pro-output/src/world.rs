use bevy::prelude::*;
use crate::voxel::{Voxel, VoxelMaterial};
use crate::chunk::{Chunk, CHUNK_SIZE};

#[derive(Resource, Default)]
pub struct VoxelWorld {
    chunks: Vec<Chunk>,
}

impl VoxelWorld {
    pub fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
}

#[derive(Debug)]
pub struct WorldPlugin;

fn generate_test_chunks(mut world: ResMut<VoxelWorld>) {
    // Generate a few test chunks
    for x in -1..=1 {
        for z in -1..=1 {
            let chunk = Chunk::new(IVec3::new(x * CHUNK_SIZE, 0, z * CHUNK_SIZE));
            world.add_chunk(chunk);
        }
    }
}

fn update_chunks(world: Res<VoxelWorld>) {
    // Placeholder system that will later handle chunk updates
    for _chunk in world.chunks.iter() {
        // No-op for now
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoxelWorld>()
           .add_systems(Startup, generate_test_chunks)
           .add_systems(Update, update_chunks);
    }
}
