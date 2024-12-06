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

fn debug_chunks(world: Res<VoxelWorld>) {
    println!("Number of chunks: {}", world.chunks.len());
}

fn remesh_chunks(mut world: ResMut<VoxelWorld>) {
    // For now, just print something when chunks need remeshing
    for chunk in &mut world.chunks {
        if chunk.needs_remesh {
            println!("Remeshing chunk at {:?}", chunk.position);
            chunk.needs_remesh = false; // reset after "remesh"
        }
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoxelWorld>()
           .add_systems(Startup, generate_test_chunks)
           .add_systems(Update, (debug_chunks, remesh_chunks));
    }
}
