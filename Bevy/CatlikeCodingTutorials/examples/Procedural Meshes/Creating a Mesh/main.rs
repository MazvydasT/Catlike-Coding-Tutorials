use std::time::Duration;

use bevy::{
    asset::RenderAssetUsages,
    core_pipeline::tonemapping::Tonemapping,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    image::ImageLoaderSettings,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    window::PresentMode,
};

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }),))
        .add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    refresh_interval: Duration::from_millis(1000),
                    frame_time_graph_config: FrameTimeGraphConfig {
                        enabled: true,
                        min_fps: 60.,
                        target_fps: 800.,
                    },
                    ..default()
                },
            },
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = meshes.add(create_quad_mesh());
    let base_map_handle: Handle<Image> = asset_server.load_with_settings(
        "textures/base-map.png",
        |settings: &mut ImageLoaderSettings| settings.is_srgb = true,
    );
    let normal_map_handle: Handle<Image> = asset_server.load_with_settings(
        "textures/normal-map.png",
        |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
    );

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(base_map_handle),
        normal_map_texture: Some(normal_map_handle),
        ..Default::default()
    });

    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_translation(Vec3::ZERO),
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 6000.,

            ..Default::default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            f32::to_radians(50.),
            f32::to_radians(30.),
            0.,
        )),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.5, 0.5, 1.)),
        Projection::from(PerspectiveProjection {
            fov: f32::to_radians(60.),
            near: 0.3,
            far: 1000.,
            ..Default::default()
        }),
        Tonemapping::None,
    ));
}

fn create_quad_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![Vec3::ZERO, Vec3::X, Vec3::Y, Vec3::new(1., 1., 0.)],
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![Vec3::Z; 4])
    .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, vec![[1., 0., 0., -1.]; 4])
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![Vec2::Y, Vec2::ONE, Vec2::ZERO, Vec2::X],
    )
    .with_inserted_indices(Indices::U16(vec![0, 1, 2, 1, 3, 2]))
}
