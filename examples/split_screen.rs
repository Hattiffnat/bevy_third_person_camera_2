use bevy::{
    camera::Viewport,
    color::palettes::tailwind::{BLUE_500, GREEN_500},
    prelude::*,
    window::WindowResized,
};
use bevy_third_person_camera_2::{self as tp_cam, SetLocalCamera, ThirdPersonCameraSettings};

#[derive(Component)]
struct MyCube;

#[derive(Default, Resource)]
struct MyCameras {
    pub cameras: Vec<Entity>,
    pub index: usize,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_resource::<MyCameras>()
        .add_plugins(tp_cam::ThirdPersonCameraPlugin::new(
            ThirdPersonCameraSettings {
                show_relation_gizmo: true,
                ..default()
            },
        ))
        .add_systems(Startup, spawn_cube_and_camera_s)
        .add_systems(Update, (move_cube_s, swap_camera_s, set_viewports_s));

    app.run();
}

fn spawn_cube_and_camera_s(
    mut commands: Commands,
    mut my_cameras: ResMut<MyCameras>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::default(),
        brightness: 1000.0,
        ..default()
    });

    commands.spawn((
        Name::new("Floor"),
        Mesh3d(meshes.add(Mesh::from(Plane3d::new(Vec3::Z, Vec2::new(10.0, 10.0))))),
        MeshMaterial3d(materials.add(Color::Srgba(GREEN_500))),
        Transform::default().looking_to(Dir3::NEG_Y, Dir3::Y),
    ));

    // Spawn some cube
    let cube = commands
        .spawn((
            Name::new("My cube"),
            MyCube,
            Transform::from_xyz(0.0, 3.0, 0.0),
            Mesh3d(meshes.add(Mesh::from(Cuboid::from_length(2.0)))),
            MeshMaterial3d(materials.add(Color::Srgba(BLUE_500))),
        ))
        .id();

    let camera_1 = commands
        .spawn((
            Name::new("MyCamera 1"),
            Camera {
                order: 0,
                ..default()
            },
            Camera3d::default(),
            Transform::from_translation(Vec3::new(-10.0, 10.0, 10.0))
                .looking_at(Vec3::ZERO, Dir3::Y),
            // Targeting to cube
            tp_cam::ThirdPersonCamera::aimed_at(cube),
        ))
        .id();

    let camera_2 = commands
        .spawn((
            Name::new("MyCamera 2"),
            Camera {
                order: 1,
                ..default()
            },
            Camera3d::default(),
            Transform::from_translation(Vec3::new(10.0, 10.0, 10.0))
                .looking_at(Vec3::ZERO, Dir3::Y),
            // Targeting to cube
            tp_cam::ThirdPersonCamera::aimed_at(cube),
        ))
        .id();

    commands.trigger(tp_cam::SetLocalCamera(camera_1));

    my_cameras.cameras.push(camera_1);
    my_cameras.cameras.push(camera_2);
}

fn set_viewports_s(
    windows: Query<&Window>,
    mut window_resized_reader: MessageReader<WindowResized>,
    my_cameras: Res<MyCameras>,
    mut cameras_q: Query<&mut Camera>,
) {
    for window_resized in window_resized_reader.read() {
        let window = windows.get(window_resized.window).unwrap();
        let x_size = window.physical_size().x / 2;

        for (pos, camera_entity) in my_cameras.cameras.iter().enumerate() {
            if let Ok(mut camera) = cameras_q.get_mut(*camera_entity) {
                camera.viewport = Some(Viewport {
                    physical_position: UVec2::new(pos as u32 * x_size, 0),
                    physical_size: window.physical_size().with_x(x_size),
                    ..default()
                });
            }
        }
    }
}

/// Swap controls to another camera
fn swap_camera_s(
    mut commands: Commands,
    mut my_cameras: ResMut<MyCameras>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyC) {
        if my_cameras.cameras.len() - 1 > my_cameras.index {
            my_cameras.index += 1;
        } else {
            my_cameras.index = 0
        }
        commands.trigger(SetLocalCamera(my_cameras.cameras[my_cameras.index]));
    }
}

/// Move the cube to demonstrate the camera's tracking
fn move_cube_s(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    cube_q: Query<&mut Transform, With<MyCube>>,
) {
    let value = time.delta_secs() * 10.0;
    for mut cube_transform in cube_q {
        if keys.pressed(KeyCode::KeyW) {
            cube_transform.translation.x += value;
        }
        if keys.pressed(KeyCode::KeyS) {
            cube_transform.translation.x -= value;
        }
        if keys.pressed(KeyCode::KeyA) {
            cube_transform.translation.z -= value;
        }
        if keys.pressed(KeyCode::KeyD) {
            cube_transform.translation.z += value;
        }
    }
}
