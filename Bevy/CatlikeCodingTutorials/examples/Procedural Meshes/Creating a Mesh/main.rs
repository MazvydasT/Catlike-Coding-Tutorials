use bevy::prelude::*;

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
    let sphere = meshes.add(Sphere::new(0.5).mesh().ico(4).unwrap());

    let white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: false,
        ..Default::default()
    });

    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(white.clone()),
        Transform::from_translation(Vec3::ZERO),
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, 3.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
        //.with_rotation(Quat::from_rotation_x(50.0))
        //.with_rotation(Quat::from_rotation_y(-30.0)),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.5, 0.5, -1.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::from(PerspectiveProjection {
            fov: 60.,
            near: 0.3,
            far: 1000.,
            ..Default::default()
        }),
    ));
}
