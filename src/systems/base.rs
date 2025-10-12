use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::{components, events, plugin_settings::ThirdPersonCameraSettings};

pub fn spawn_components_s(
    mut commands: Commands,
    tp_cam_settings: Res<ThirdPersonCameraSettings>,
    tp_cam_q: Query<
        (
            Entity,
            &components::ThirdPersonCamera,
            Has<components::CameraOffset>,
            Has<components::TargetOffset>,
            Has<components::TargetPoint>,
            Has<components::DampingFactor>,
        ),
        Added<components::ThirdPersonCamera>,
    >,
    target_transform_q: Query<&Transform, With<components::ThirdPersonCameraTarget>>,
) {
    for (tp_cam_entity, tp_cam, has_cam_offset, has_target_offset, has_target_point, has_damping) in
        tp_cam_q
    {
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
        if !has_target_point {
            if let Ok(target_transform) = target_transform_q.get(tp_cam.target) {
                commands
                    .entity(tp_cam_entity)
                    .insert(components::TargetPoint(
                        target_transform.translation + tp_cam_settings.default_target_offset,
                    ));
            } else {
                error!("{} query failed {:?}", tp_cam.target, target_transform_q);
            }
        }
        tp_cam_settings.default_damping.inspect(|damping_factor| {
            if !has_damping {
                commands
                    .entity(tp_cam_entity)
                    .insert(components::DampingFactor(*damping_factor));
            }
        });
    }
}

pub fn calculate_target_point_s(
    time: Res<Time>,
    target_transform_q: Query<
        (&Transform, &components::ThirdPersonCameraTarget),
        With<components::ThirdPersonCameraTarget>,
    >,
    mut camera_transform_q: Query<
        (
            &components::TargetOffset,
            &mut components::TargetPoint,
            Option<&components::DampingFactor>,
        ),
        Without<components::ThirdPersonCameraTarget>,
    >,
) {
    for (target_transform, target) in target_transform_q {
        for camera_entity in target.iter() {
            if let Ok((target_offset, mut target_point, damping_op)) =
                camera_transform_q.get_mut(camera_entity)
            {
                let absolute = target_transform.translation + target_offset.0;

                target_point.0 = damping_op.map_or(absolute, |damping_factor| {
                    target_point
                        .0
                        .lerp(absolute, time.delta_secs() * damping_factor.0)
                })
            }
        }
    }
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
