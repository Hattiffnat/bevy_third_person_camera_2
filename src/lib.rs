use bevy::prelude::*;

use crate::systems::*;
pub use components::*;
pub use events::*;
pub use plugin_settings::ThirdPersonCameraSettings;

mod components;
mod events;
mod observers;
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
            .add_observer(observers::rotate_camera_o)
            .add_observer(observers::adjust_translation_o)
            .add_observer(observers::set_local_cam_s)
            .add_observer(observers::roll_camera_s)
            .add_observer(observers::zoom_s)
            .add_systems(PreUpdate, spawn_components_s)
            .add_systems(
                Update,
                (
                    calculate_target_point_s,
                    adjust_translation_after_target_s,
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
