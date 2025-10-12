use bevy::color::palettes::css::WHITE;

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
