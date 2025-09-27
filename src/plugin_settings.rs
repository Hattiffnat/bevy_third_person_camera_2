use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy)]
pub struct ThirdPersonCameraSettings {
    /// The position of the camera relative to the target point.
    /// Can be changed for a specific camera using the СameraOffset component.
    pub default_camera_offset: Vec3,

    /// Shift the target point relative to the target coordinates.
    /// For example, if the character's coordinate point is at the feet,
    /// shift the rotation point, Y-wise, to the character's chest.
    /// Can be changed for a specific camera using the TargetOffset component.
    pub default_target_offset: Vec3,

    pub pitch_max: f32,
    pub pitch_min: f32,

    pub local_cam: Option<Entity>,

    pub cam_speed: f32,
    pub mouse_speed: f32,

    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,

    pub roll_clockwise: KeyCode,
    pub roll_counterclockwise: KeyCode,
}

impl Default for ThirdPersonCameraSettings {
    fn default() -> Self {
        Self {
            cam_speed: 1.0,
            mouse_speed: 0.005,

            default_camera_offset: Vec3::ZERO.with_z(-15.0),
            default_target_offset: Vec3::ZERO.with_y(0.0),

            pitch_max: 89f32.to_radians(),
            pitch_min: -89f32.to_radians(),

            local_cam: None,

            up: KeyCode::ArrowUp,
            down: KeyCode::ArrowDown,
            left: KeyCode::ArrowLeft,
            right: KeyCode::ArrowRight,

            roll_clockwise: KeyCode::KeyE,
            roll_counterclockwise: KeyCode::KeyQ,
        }
    }
}
