use bevy::prelude::*;

use crate::systems::*;
pub use plugin_settings::ThirdPersonCameraSettings;

pub use components::*;
pub use events::*;

mod components;
mod events;
mod plugin_settings;
mod systems;

pub struct ThirdPersonCameraPlugin {
    settings: ThirdPersonCameraSettings,
}

impl ThirdPersonCameraPlugin {
    pub fn new(settings: ThirdPersonCameraSettings) -> Self {
        Self { settings }
    }
}

impl Default for ThirdPersonCameraPlugin {
    fn default() -> Self {
        Self {
            settings: ThirdPersonCameraSettings::default(),
        }
    }
}

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone())
            .add_event::<events::SetLocalCamera>()
            .add_event::<events::RotateAroundTarget>()
            .add_event::<events::Roll>()
            .add_event::<events::Zoom>()
            .add_systems(PreUpdate, spawn_offset_s)
            .add_systems(
                Update,
                (
                    (rotate_camera_s, translate_camera_s).chain(),
                    zoom_s,
                    roll_camera_s,
                    set_local_cam,
                    draw_relation_gizmo_s,
                ),
            )
            .add_systems(
                Update,
                (
                    mouse_rotation_control_s,
                    keyboard_rotation_control_s,
                    scroll_zoom_s,
                ),
            );
    }
}
