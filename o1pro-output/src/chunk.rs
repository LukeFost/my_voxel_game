use bevy::prelude::*;
use crate::voxel::VoxelMaterial;

pub const CHUNK_SIZE: i32 = 16;
const CHUNK_VOLUME: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

/// Represents a cubic section of the voxel world
#[derive(Debug, Component)]
pub struct Chunk {
    /// Position in chunk coordinates (multiply by CHUNK_SIZE for world coordinates)
    pub position: IVec3,
    /// The voxel data for this chunk
    pub voxels: Box<[VoxelMaterial]>,
    /// Whether this chunk needs its mesh regenerated
    pub needs_remesh: bool,
    /// Last time this chunk was accessed (for unload decisions)
    pub last_accessed: f64,
}

impl Chunk {
    pub fn new(position: IVec3) -> Self {
        Self {
            position,
            voxels: vec![VoxelMaterial::Air; CHUNK_VOLUME].into_boxed_slice(),
            needs_remesh: true,
            last_accessed: 0.0,
        }
    }

    /// Convert world coordinates to chunk-local coordinates
    pub fn world_to_local(world_pos: IVec3) -> (IVec3, IVec3) {
        let chunk_pos = IVec3::new(
            (world_pos.x.div_euclid(CHUNK_SIZE)),
            (world_pos.y.div_euclid(CHUNK_SIZE)),
            (world_pos.z.div_euclid(CHUNK_SIZE))
        );
        
        let local_pos = IVec3::new(
            world_pos.x.rem_euclid(CHUNK_SIZE),
            world_pos.y.rem_euclid(CHUNK_SIZE),
            world_pos.z.rem_euclid(CHUNK_SIZE)
        );
        
        (chunk_pos, local_pos)
    }

    /// Get the index into the voxel array for a local position
    pub fn get_voxel_index(&self, local_pos: IVec3) -> Option<usize> {
        if local_pos.x < 0 || local_pos.y < 0 || local_pos.z < 0 
            || local_pos.x >= CHUNK_SIZE 
            || local_pos.y >= CHUNK_SIZE 
            || local_pos.z >= CHUNK_SIZE {
            return None;
        }
        
        Some((local_pos.x + local_pos.y * CHUNK_SIZE + local_pos.z * CHUNK_SIZE * CHUNK_SIZE) as usize)
    }

    /// Get a voxel at a local position
    pub fn get_voxel(&self, local_pos: IVec3) -> Option<VoxelMaterial> {
        self.get_voxel_index(local_pos)
            .map(|idx| self.voxels[idx])
    }

    /// Set a voxel at a local position
    pub fn set_voxel(&mut self, local_pos: IVec3, material: VoxelMaterial) -> bool {
        if let Some(idx) = self.get_voxel_index(local_pos) {
            self.voxels[idx] = material;
            self.needs_remesh = true;
            true
        } else {
            false
        }
    }

    /// Convert local coordinates to world coordinates
    pub fn local_to_world(&self, local_pos: IVec3) -> IVec3 {
        self.position * CHUNK_SIZE + local_pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_conversion() {
        let world_pos = IVec3::new(35, -12, 67);
        let (chunk_pos, local_pos) = Chunk::world_to_local(world_pos);
        
        let chunk = Chunk::new(chunk_pos);
        let reconstructed = chunk.local_to_world(local_pos);
        
        assert_eq!(world_pos, reconstructed);
    }

    #[test]
    fn test_voxel_access() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        let pos = IVec3::new(1, 2, 3);
        
        assert!(chunk.set_voxel(pos, VoxelMaterial::Stone));
        assert_eq!(chunk.get_voxel(pos), Some(VoxelMaterial::Stone));
    }
}
