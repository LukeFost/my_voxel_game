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

fn update_voxels(time: Res<Time>, mut query: Query<&mut Transform, With<Voxel>>) {
    // Placeholder system that will later handle voxel updates
    for _transform in query.iter_mut() {
        // No-op for now
    }
}

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_voxels);
    }
}
