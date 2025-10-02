use bevy::prelude::*;

/// Assign the camera to be controlled locally
#[derive(Debug, EntityEvent)]
pub struct SetLocalCamera(pub Entity);

/// Rotate camera around target
#[derive(Debug, EntityEvent)]
pub struct RotateAroundTarget {
    #[event_target]
    pub camera: Entity,
    pub delta: Vec2,
}

#[derive(Debug, EntityEvent)]
pub struct AdjustTranslation {
    #[event_target]
    pub camera: Entity,
}

/// Zoom or zoom out (if the value is negative)
#[derive(Debug, EntityEvent)]
pub struct Zoom {
    #[event_target]
    pub camera: Entity,
    pub value: f32,
}

/// Roll camera
#[derive(Debug, EntityEvent)]
pub struct Roll {
    #[event_target]
    pub camera: Entity,
    pub value: f32,
}
