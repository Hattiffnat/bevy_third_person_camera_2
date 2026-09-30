use bevy::{color::palettes::css, prelude::*};
use bevy_third_person_camera_2 as tp_cam;

#[derive(Component)]
struct MyCube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(tp_cam::ThirdPersonCameraPlugin::default())
        .add_systems(Startup, spawn_cube_and_camera_s)
        .add_systems(Update, move_cube_s)
        .run();
}

fn spawn_cube_and_camera_s(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    // background
    commands.insert_resource(ClearColor(css::SKY_BLUE.into()));

    // light
    *ambient = GlobalAmbientLight::NONE;
    commands.spawn((
        DirectionalLight {
            color: css::WHITE.into(),
            illuminance: 1000.,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // floor
    commands.spawn((
        Name::new("Floor"),
        Mesh3d(meshes.add(Mesh::from(Plane3d::new(Vec3::Z, Vec2::new(10.0, 10.0))))),
        MeshMaterial3d(materials.add(Color::Srgba(css::LIGHT_GREEN))),
        Transform::default().looking_to(Dir3::NEG_Y, Dir3::Y),
    ));

    // some cube
    let cube = commands
        .spawn((
            Name::new("My cube"),
            MyCube,
            Transform::from_xyz(0.0, 1.0, 0.0),
            Mesh3d(meshes.add(Mesh::from(Cuboid::from_length(2.0)))),
            MeshMaterial3d(materials.add(Color::Srgba(css::ORANGE_RED))),
        ))
        .id();

    // camera aimed on cube
    let camera = commands
        .spawn((
            Name::new("MyCamera"),
            Camera3d::default(),
            Transform::default(),
            // Targeting to cube
            tp_cam::ThirdPersonCamera::aimed_at(cube),
            // Damping
            tp_cam::DampingFactor(5.0),
            AmbientLight {
                color: css::WHITE.into(),
                brightness: 500.0,
                affects_lightmapped_meshes: true,
                ..default()
            },
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
