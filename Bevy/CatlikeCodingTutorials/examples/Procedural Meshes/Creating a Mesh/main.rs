use bevy::prelude::*;

#[path = "../../../src/utils.rs"]
mod utils;
use utils::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(0.5).mesh().ico(4).unwrap());

    let white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: false,
        ..Default::default()
    });

    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(white.clone()),
        Transform::from_translation(Vec3::ZERO),
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            ..Default::default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::ZXY,
            deg2rad(50.),
            deg2rad(-30.),
            0.,
        )),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        Projection::from(PerspectiveProjection {
            fov: deg2rad(60.),
            near: 0.3,
            far: 1000.,
            ..Default::default()
        }),
    ));
}
