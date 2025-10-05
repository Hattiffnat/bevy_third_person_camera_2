use bevy::{
    color::palettes::tailwind::{BLUE_500, GREEN_500},
    prelude::*,
};
use bevy_third_person_camera_2 as tp_cam;

#[derive(Component)]
struct MyCube;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(tp_cam::ThirdPersonCameraPlugin::default())
        .add_systems(Startup, spawn_cube_and_camera_s)
        .add_systems(Update, move_cube_s);

    app.run();
}

fn spawn_cube_and_camera_s(
    mut commands: Commands,
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

    let camera = commands
        .spawn((
            Name::new("MyCamera"),
            Camera3d::default(),
            Transform::default(),
            // Targeting to cube
            tp_cam::ThirdPersonCamera::aimed_at(cube),
            // Damping
            tp_cam::DampingFactor(5.0),
        ))
        .id();

    // There can be multiple cameras in a scene, so we explicitly assign
    // this one to be controlled by the keyboard and mouse.
    commands.trigger(tp_cam::SetLocalCamera(camera));
    // Alternatively, you can fine-tune your controls using the events provided by this plugin.
}

/// Move the cube to demonstrate the camera's tracking and damping
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
