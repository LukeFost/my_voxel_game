use bevy::prelude::*;
use crate::voxel::{Voxel, VoxelMaterial};

pub const CHUNK_SIZE: i32 = 16;

#[derive(Resource)]
pub struct VoxelWorld {
    chunks: Vec<Chunk>,
}

pub struct Chunk {
    position: IVec3,
    voxels: Vec<VoxelMaterial>,
}

impl Chunk {
    pub fn new(position: IVec3) -> Self {
        Self {
            position,
            voxels: vec![VoxelMaterial::Air; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize],
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoxelWorld>()
           .add_systems(Startup, setup_world)
           .add_systems(Update, (
               // We'll add systems here later
           ));
    }
}

fn setup_world(mut commands: Commands) {
    // Initial world setup will go here
}
