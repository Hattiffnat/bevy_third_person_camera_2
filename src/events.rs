use bevy::prelude::*;

/// Assign the camera to be controlled locally
#[derive(Debug, Event)]
pub struct SetLocalCam(pub Entity);

/// Rotate camera around target
#[derive(Debug, Event)]
pub struct RotateAroundTarget {
    pub camera: Entity,
    pub delta: Vec2,
}

/// Zoom or zoom out (if the value is negative)
#[derive(Debug, Event)]
pub struct Zoom {
    pub camera: Entity,
    pub value: f32,
}

/// Roll camera
#[derive(Debug, Event)]
pub struct Roll {
    pub camera: Entity,
    pub value: f32,
}
