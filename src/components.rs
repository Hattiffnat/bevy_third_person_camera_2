use bevy::prelude::*;

/// Position of the camera relative to the target point.
#[derive(Component)]
pub struct CameraOffset(pub Vec3);

/// Shift the target point relative to the target coordinates.
/// For example, if the character's coordinate point is at the feet,
/// shift the rotation point, Y-wise, to the character's chest.
#[derive(Component)]
pub struct TargetOffset(pub Vec3);

/// Delays camera tracking if inserted. The lower the value, the greater the delay.
#[derive(Component)]
pub struct DampingFactor(pub f32);

/// Cached target point and the target's inherited XZ-plane tilt (excluding local-Y rotation).
#[derive(Component)]
pub struct TargetPoint {
    pub position: Vec3,
    pub rotation: Quat,
}

#[derive(Component)]
#[relationship(relationship_target = ThirdPersonCameraTarget)]
pub struct ThirdPersonCamera {
    #[relationship]
    pub target: Entity,
}

impl ThirdPersonCamera {
    pub fn aimed_at(target: Entity) -> Self {
        Self { target }
    }
}

#[derive(Component)]
#[relationship_target(relationship = ThirdPersonCamera)]
pub struct ThirdPersonCameraTarget(Vec<Entity>);
