use bevy::{
    color::palettes::css::WHITE,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::{components, events, plugin_settings::ThirdPersonCameraSettings};

pub fn spawn_offset_s(
    mut commands: Commands,
    tp_cam_settings: Res<ThirdPersonCameraSettings>,
    tp_cam_q: Query<
        (
            Entity,
            Has<components::CameraOffset>,
            Has<components::TargetOffset>,
        ),
        Added<components::ThirdPersonCamera>,
    >,
) {
    for (tp_cam_entity, has_cam_offset, has_target_offset) in tp_cam_q {
        if !has_cam_offset {
            commands
                .entity(tp_cam_entity)
                .insert(components::CameraOffset(
                    tp_cam_settings.default_camera_offset,
                ));
        }
        if !has_target_offset {
            commands
                .entity(tp_cam_entity)
                .insert(components::TargetOffset(
                    tp_cam_settings.default_target_offset,
                ));
        }
    }
}

pub fn mouse_rotation_control_s(
    mut commands: Commands,
    mouse_motion: Res<AccumulatedMouseMotion>,
    camera_settings: Res<ThirdPersonCameraSettings>,
) {
    if let Some(camera) = camera_settings.local_cam {
        if mouse_motion.is_changed() {
            commands.trigger(events::RotateAroundTarget {
                camera,
                delta: mouse_motion.delta * camera_settings.mouse_speed,
            });
        }
    };
}

pub fn keyboard_rotation_control_s(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    camera_settings: Res<ThirdPersonCameraSettings>,
) {
    if let Some(camera) = camera_settings.local_cam {
        let mut yaw = 0.0;
        let mut pitch = 0.0;

        if keys.pressed(camera_settings.up) {
            pitch -= camera_settings.cam_speed * time.delta_secs();
        }
        if keys.pressed(camera_settings.down) {
            pitch += camera_settings.cam_speed * time.delta_secs();
        }
        pitch = pitch.clamp(camera_settings.pitch_min, camera_settings.pitch_max);

        if keys.pressed(camera_settings.left) {
            yaw -= camera_settings.cam_speed * time.delta_secs();
        }
        if keys.pressed(camera_settings.right) {
            yaw += camera_settings.cam_speed * time.delta_secs();
        }
        if yaw != 0.0 || pitch != 0.0 {
            commands.trigger(events::RotateAroundTarget {
                camera: camera,
                delta: Vec2::new(yaw, pitch),
            });
        }

        let mut roll = 0.0;
        if keys.pressed(camera_settings.roll_clockwise) {
            debug!("clockwise");
            roll += camera_settings.cam_speed * time.delta_secs()
        } else if keys.pressed(camera_settings.roll_counterclockwise) {
            debug!("counterclockwise");
            roll -= camera_settings.cam_speed * time.delta_secs()
        }
        if roll != 0.0 {
            commands.trigger(events::Roll {
                camera,
                value: roll,
            });
        }
    };
}

pub fn adjust_translation_after_target_s(
    mut commands: Commands,
    target_q: Query<&components::ThirdPersonCameraTarget, Changed<Transform>>,
) {
    for third_person_camera in target_q {
        for camera in third_person_camera.iter() {
            commands.trigger(events::AdjustTranslation { camera });
        }
    }
}

pub fn scroll_zoom_s(
    mut commands: Commands,
    wheel_move: Res<AccumulatedMouseScroll>,
    third_person_cam_settings: Res<ThirdPersonCameraSettings>,
) {
    if let Some(camera) = third_person_cam_settings.local_cam {
        commands.trigger(events::Zoom {
            camera,
            value: wheel_move.delta.y,
        });
    };
}

pub fn draw_relation_gizmo_s(
    mut gizmos: Gizmos,
    third_person_cam_settings: Res<ThirdPersonCameraSettings>,
    target_global_transf_q: Query<
        (&GlobalTransform, &components::ThirdPersonCameraTarget),
        Without<components::ThirdPersonCamera>,
    >,
    camera_global_transf_q: Query<
        (&GlobalTransform, &components::TargetOffset),
        With<components::ThirdPersonCamera>,
    >,
) {
    if !third_person_cam_settings.show_relation_gizmo {
        return;
    }

    for (target_global_transf, third_person_cam_target) in target_global_transf_q.iter() {
        let target_transf = target_global_transf.compute_transform();
        for camera_entity in third_person_cam_target.iter() {
            if let Ok((camera_global_transf, target_offset)) =
                camera_global_transf_q.get(camera_entity)
            {
                gizmos.line(
                    target_transf.translation + target_offset.0,
                    camera_global_transf.compute_transform().translation,
                    WHITE,
                );
            }
        }
    }
}
