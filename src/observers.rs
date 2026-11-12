use bevy::prelude::*;

use crate::{ThirdPersonCameraSettings, components, events};

pub fn rotate_camera_o(
    rotate_ev: On<events::RotateAroundTarget>,
    mut commands: Commands,
    tp_cam_settings: Res<ThirdPersonCameraSettings>,
    mut camera_q: Query<
        (&mut Transform, &components::TargetPoint),
        With<components::ThirdPersonCamera>,
    >,
) {
    let Ok((mut camera_transform, target_point)) = camera_q.get_mut(rotate_ev.camera) else {
        return;
    };

    let target_rotation = target_point.rotation;
    let relative_rotation = target_rotation.inverse() * camera_transform.rotation;
    let (mut yaw, mut pitch, roll) = relative_rotation.to_euler(EulerRot::YXZ);

    yaw -= tp_cam_settings.cam_speed * rotate_ev.delta.x;
    pitch = (pitch - tp_cam_settings.cam_speed * rotate_ev.delta.y)
        .clamp(tp_cam_settings.pitch_min, tp_cam_settings.pitch_max);

    camera_transform.rotation = target_rotation * Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    commands.trigger(events::AdjustTranslation {
        camera: rotate_ev.camera,
    });
}

pub fn adjust_translation_o(
    adjust_translation_ev: On<events::AdjustTranslation>,
    mut camera_transform_q: Query<
        (
            &mut Transform,
            &components::CameraOffset,
            &components::TargetPoint,
        ),
        With<components::ThirdPersonCamera>,
    >,
) {
    if let Ok((mut camera_transform, camera_offset, target_point)) =
        camera_transform_q.get_mut(adjust_translation_ev.camera)
    {
        camera_transform.translation =
            target_point.position - camera_transform.rotation * camera_offset.0;
    } else {
        error!(
            "{} query failed {:?}",
            adjust_translation_ev.camera, camera_transform_q
        );
    }
}

pub fn zoom_o(
    zoom_ev: On<events::Zoom>,
    camera_settings: Res<ThirdPersonCameraSettings>,
    mut third_person_camera_q: Query<&mut components::CameraOffset>,
) {
    if let Ok(mut cam_offset) = third_person_camera_q.get_mut(zoom_ev.camera) {
        cam_offset.0.z += zoom_ev.value * camera_settings.cam_speed
    } else {
        error!(
            "{} query failed {:?}",
            zoom_ev.camera, third_person_camera_q
        );
    }
}

pub fn roll_camera_o(
    roll_event: On<events::Roll>,
    camera_settings: Res<ThirdPersonCameraSettings>,
    mut tp_cam_transform: Query<&mut Transform, With<components::ThirdPersonCamera>>,
) {
    if let Ok(mut cam_transform) = tp_cam_transform.get_mut(roll_event.camera) {
        debug!("roll_event: {:?}", roll_event);
        cam_transform.rotate_local_z(roll_event.value * camera_settings.cam_speed);
    }
}

pub fn set_local_cam_o(
    set_local_cam_ev: On<events::SetLocalCamera>,
    mut camera_settings: ResMut<ThirdPersonCameraSettings>,
) {
    camera_settings.local_cam = Some(set_local_cam_ev.0)
}
