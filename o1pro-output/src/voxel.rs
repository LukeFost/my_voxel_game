use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Voxel {
    pub material: VoxelMaterial,
    pub position: IVec3,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum VoxelMaterial {
    Air,
    Dirt,
    Stone,
    Grass,
}

impl Default for VoxelMaterial {
    fn default() -> Self {
        Self::Air
    }
}

#[derive(Debug)]
pub struct VoxelPlugin;

fn debug_voxels(query: Query<&Voxel>) {
    // Print how many voxels we have, just as a placeholder
    println!("Voxel count: {}", query.iter().count());
}

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_voxels);
    }
}
