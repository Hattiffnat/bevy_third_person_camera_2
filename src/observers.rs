use bevy::prelude::*;

use crate::{ThirdPersonCameraSettings, components, events};

pub fn rotate_camera_o(
    rotate_ev: On<events::RotateAroundTarget>,
    mut commands: Commands,
    camera_settings: Res<ThirdPersonCameraSettings>,
    mut camera_transform_q: Query<&mut Transform, With<components::ThirdPersonCamera>>,
) {
    let Ok(mut camera_transform) = camera_transform_q.get_mut(rotate_ev.camera) else {
        return;
    };

    let (mut yaw, mut pitch, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);

    yaw -= camera_settings.cam_speed * rotate_ev.delta.x;

    pitch = (pitch - camera_settings.cam_speed * rotate_ev.delta.y)
        .clamp(camera_settings.pitch_min, camera_settings.pitch_max);

    camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
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
        camera_transform.translation = target_point.0 - camera_transform.rotation * camera_offset.0;
    } else {
        error!(
            "{} query failed {:?}",
            adjust_translation_ev.camera, camera_transform_q
        );
    }
}

pub fn zoom_s(
    zoom_ev: On<events::Zoom>,
    mut third_person_camera_q: Query<&mut components::CameraOffset>,
) {
    if let Ok(mut cam_offset) = third_person_camera_q.get_mut(zoom_ev.camera) {
        cam_offset.0.z += zoom_ev.value
    } else {
        error!(
            "{} query failed {:?}",
            zoom_ev.camera, third_person_camera_q
        );
    }
}

pub fn roll_camera_s(
    roll_event: On<events::Roll>,
    mut tp_cam_transform: Query<&mut Transform, With<components::ThirdPersonCamera>>,
) {
    if let Ok(mut cam_transform) = tp_cam_transform.get_mut(roll_event.camera) {
        debug!("roll_event: {:?}", roll_event);
        cam_transform.rotate_local_z(roll_event.value);
    }
}

pub fn set_local_cam_s(
    set_local_cam_ev: On<events::SetLocalCamera>,
    mut camera_settings: ResMut<ThirdPersonCameraSettings>,
) {
    camera_settings.local_cam = Some(set_local_cam_ev.0)
}
