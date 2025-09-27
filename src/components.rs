use bevy::prelude::*;

/// The position of the camera relative to the target point
#[derive(Component)]
pub struct CameraOffset(pub Vec3);

/// Shift the target point relative to the target coordinates.
/// For example, if the character's coordinate point is at the feet,
/// shift the rotation point, Y-wise, to the character's chest.
#[derive(Component)]
pub struct TargetOffset(pub Vec3);

#[derive(Component)]
#[relationship(relationship_target = ThirdPersonCamTarget)]
pub struct ThirdPersonCamera {
    #[relationship]
    pub target: Entity,
}

impl ThirdPersonCamera {
    pub fn new(target: Entity) -> Self {
        Self { target }
    }
}

#[derive(Component)]
#[relationship_target(relationship = ThirdPersonCamera)]
pub struct ThirdPersonCamTarget(Vec<Entity>);
