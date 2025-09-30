use bevy::{
    color::palettes::css::WHITE,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::{
    components::{CameraOffset, TargetOffset, ThirdPersonCamera, ThirdPersonCameraTarget},
    events::{Roll, RotateAroundTarget, SetLocalCamera, Zoom},
    plugin_settings::ThirdPersonCameraSettings,
};

pub fn spawn_offset_s(
    mut commands: Commands,
    tp_cam_settings: Res<ThirdPersonCameraSettings>,
    tp_cam_q: Query<Entity, (Added<ThirdPersonCamera>, Without<CameraOffset>)>,
) {
    for tp_cam_entity in tp_cam_q {
        commands.entity(tp_cam_entity).insert((
            CameraOffset(tp_cam_settings.default_camera_offset),
            TargetOffset(tp_cam_settings.default_target_offset),
        ));
    }
}

pub fn rotate_camera_s(
    mut rotate_er: EventReader<RotateAroundTarget>,
    camera_settings: Res<ThirdPersonCameraSettings>,
    mut camera_transform_q: Query<&mut Transform, With<ThirdPersonCamera>>,
) {
    for rotate_ev in rotate_er.read() {
        let Ok(mut camera_transform) = camera_transform_q.get_mut(rotate_ev.camera) else {
            return;
        };

        let (mut yaw, mut pitch, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= camera_settings.cam_speed * rotate_ev.delta.x;

        pitch = (pitch - camera_settings.cam_speed * rotate_ev.delta.y)
            .clamp(camera_settings.pitch_min, camera_settings.pitch_max);

        camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

pub fn roll_camera_s(
    mut roll_er: EventReader<Roll>,
    mut tp_cam_transform: Query<&mut Transform, With<ThirdPersonCamera>>,
) {
    for roll_event in roll_er.read() {
        if let Ok(mut cam_transform) = tp_cam_transform.get_mut(roll_event.camera) {
            debug!("roll_event: {:?}", roll_event);
            cam_transform.rotate_local_z(roll_event.value);
        }
    }
}

pub fn translate_camera_s(
    target_transform_q: Query<
        &Transform,
        (With<ThirdPersonCameraTarget>, Without<ThirdPersonCamera>),
    >,
    mut camera_transform_q: Query<
        (
            &mut Transform,
            &ThirdPersonCamera,
            &CameraOffset,
            &TargetOffset,
        ),
        Without<ThirdPersonCameraTarget>,
    >,
) {
    camera_transform_q.par_iter_mut().for_each(
        |(mut camera_transform, third_person_camera, cam_offset, target_offset)| {
            let Ok(target_transform) = target_transform_q.get(third_person_camera.target) else {
                error!(
                    "{} query failed {:?}",
                    third_person_camera.target, target_transform_q
                );
                return;
            };

            let target_point = target_transform.translation + target_offset.0;
            camera_transform.translation = target_point - camera_transform.rotation * cam_offset.0;
        },
    );
}

pub fn zoom_s(mut zoom_er: EventReader<Zoom>, mut third_person_camera_q: Query<&mut CameraOffset>) {
    for zoom_event in zoom_er.read() {
        if let Ok(mut cam_offset) = third_person_camera_q.get_mut(zoom_event.camera) {
            cam_offset.0.z += zoom_event.value
        } else {
            error!(
                "{} query failed {:?}",
                zoom_event.camera, third_person_camera_q
            );
        }
    }
}

pub fn mouse_rotation_control_s(
    mouse_motion: Res<AccumulatedMouseMotion>,
    camera_settings: Res<ThirdPersonCameraSettings>,
    mut rotate_ew: EventWriter<RotateAroundTarget>,
) {
    if let Some(camera) = camera_settings.local_cam {
        rotate_ew.write(RotateAroundTarget {
            camera,
            delta: mouse_motion.delta * camera_settings.mouse_speed,
        });
    };
}

pub fn keyboard_rotation_control_s(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    camera_settings: Res<ThirdPersonCameraSettings>,
    mut rotate_ew: EventWriter<RotateAroundTarget>,
    mut roll_ew: EventWriter<Roll>,
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
            rotate_ew.write(RotateAroundTarget {
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
            roll_ew.write(Roll {
                camera,
                value: roll,
            });
        }
    };
}

pub fn scroll_zoom_s(
    wheel_move: Res<AccumulatedMouseScroll>,
    third_person_cam_settings: Res<ThirdPersonCameraSettings>,
    mut zoom_ew: EventWriter<Zoom>,
) {
    if let Some(camera) = third_person_cam_settings.local_cam {
        zoom_ew.write(Zoom {
            camera,
            value: wheel_move.delta.y,
        });
    };
}

pub fn set_local_cam(
    mut set_local_cam_er: EventReader<SetLocalCamera>,
    mut camera_settings: ResMut<ThirdPersonCameraSettings>,
) {
    for set_local_cam_event in set_local_cam_er.read() {
        camera_settings.local_cam = Some(set_local_cam_event.0)
    }
}

pub fn draw_relation_gizmo_s(
    mut gizmos: Gizmos,
    third_person_cam_settings: Res<ThirdPersonCameraSettings>,
    target_global_transf_q: Query<
        (&GlobalTransform, &ThirdPersonCameraTarget),
        Without<ThirdPersonCamera>,
    >,
    camera_global_transf_q: Query<(&GlobalTransform, &TargetOffset), With<ThirdPersonCamera>>,
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
