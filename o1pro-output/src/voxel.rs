use bevy::prelude::*;

#[derive(Component)]
pub struct Voxel {
    pub material: VoxelMaterial,
    pub position: IVec3,
}

#[derive(Clone, Copy, PartialEq)]
pub enum VoxelMaterial {
    Air,
    Dirt,
    Stone,
    Grass,
}

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            // We'll add systems here later
        ));
    }
}
