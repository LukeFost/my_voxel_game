use bevy::prelude::*;
use crate::voxel::VoxelMaterial;

pub const CHUNK_SIZE: i32 = 16;
const CHUNK_VOLUME: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

#[derive(Debug)]
pub struct Chunk {
    pub position: IVec3,
    pub voxels: Box<[VoxelMaterial]>,
    pub needs_remesh: bool,
}

impl Chunk {
    pub fn new(position: IVec3) -> Self {
        Self {
            position,
            voxels: vec![VoxelMaterial::Air; CHUNK_VOLUME].into_boxed_slice(),
            needs_remesh: true,
        }
    }

    pub fn get_voxel_index(&self, local_pos: IVec3) -> Option<usize> {
        if local_pos.x < 0 || local_pos.y < 0 || local_pos.z < 0 
            || local_pos.x >= CHUNK_SIZE 
            || local_pos.y >= CHUNK_SIZE 
            || local_pos.z >= CHUNK_SIZE {
            return None;
        }
        
        Some((local_pos.x + local_pos.y * CHUNK_SIZE + local_pos.z * CHUNK_SIZE * CHUNK_SIZE) as usize)
    }
}
