use bevy::prelude::*;

/// Position of the camera relative to the target point.
#[derive(Component)]
pub struct CameraOffset(pub Vec3);

/// Shift the target point relative to the target coordinates.
/// For example, if the character's coordinate point is at the feet,
/// shift the rotation point, Y-wise, to the character's chest.
#[derive(Component)]
pub struct TargetOffset(pub Vec3);

#[derive(Component)]
#[relationship(relationship_target = ThirdPersonCameraTarget)]
pub struct ThirdPersonCamera {
    #[relationship]
    pub target: Entity,
}

impl ThirdPersonCamera {
    #[deprecated(
        since = "0.1.2",
        note = "the method name is not informative, use \"aimed_at(target)\""
    )]
    pub fn new(target: Entity) -> Self {
        Self { target }
    }
    pub fn aimed_at(target: Entity) -> Self {
        Self { target }
    }
}

#[derive(Component)]
#[relationship_target(relationship = ThirdPersonCamera)]
pub struct ThirdPersonCameraTarget(Vec<Entity>);
