use std::{
    f32::consts::TAU,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    camera::{Camera3d, PerspectiveProjection, Projection, visibility::Visibility},
    color::{Color, Srgba},
    core_pipeline::tonemapping::Tonemapping,
    ecs::{
        children,
        component::Component,
        resource::Resource,
        schedule::{IntoScheduleConfigs, SystemCondition, common_conditions::run_once},
        system::{Commands, Query, Res, ResMut},
    },
    light::{DirectionalLight, SpotLight},
    math::{
        Curve, Dir3, FloatExt, Quat, Vec3,
        curve::{EaseFunction, EasingCurve},
        primitives::{Cuboid, Cylinder},
    },
    mesh::{Mesh, Mesh3d, MeshBuilder, Meshable},
    pbr::{MeshMaterial3d, StandardMaterial},
    time::{Time, Timer, TimerMode, common_conditions::on_timer},
    transform::components::Transform,
};
use time::{OffsetDateTime, Time as OffsetTime};

#[derive(Resource, Default)]
struct PreviousTime(Option<OffsetTime>);

#[derive(Resource, Default)]
struct RunAnimations(bool);

#[derive(Component)]
enum TimeComponentType {
    Hour,
    Minute,
    Second,
}

#[derive(Component)]
struct AnimationEasingCurve(EasingCurve<f32>);

#[derive(Component, Default)]
struct AnimationTimer(Timer);

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(RunAnimations::default())
            .insert_resource(PreviousTime::default())
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                (
                    set_animation_timers.run_if(run_once.or_else(on_timer(Duration::from_secs(1)))),
                    animate_arms.run_if(|run_animations: Res<RunAnimations>| run_animations.0),
                )
                    .chain(),
            );
    }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let face_mesh_handle = meshes.add(Cylinder::new(5., 0.2).mesh().build());
    let face_material_handle = materials.add(StandardMaterial {
        base_color: Srgba::hex("#808080").unwrap().into(),

        ..Default::default()
    });

    commands.spawn((
        Mesh3d(face_mesh_handle),
        MeshMaterial3d(face_material_handle),
        Transform::from_rotation(Quat::from_rotation_x(90_f32.to_radians())),
    ));

    let unit_cube_mesh = Mesh3d(meshes.add(Cuboid::new(1., 1., 1.).mesh().build()));

    let indicator_material = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("494949").unwrap().into(),

        ..Default::default()
    }));

    commands.spawn_batch(
        (0..12)
            .map(|hour_indicator_index| {
                let mut transform = Transform::from_translation(Vec3::new(0., 4., 0.25))
                    .with_scale(Vec3::new(0.5, 1., 0.1));

                transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_rotation_z(TAU / 12. * (hour_indicator_index as f32)),
                );

                (
                    unit_cube_mesh.clone(),
                    indicator_material.clone(),
                    transform,
                )
            })
            .collect::<Vec<_>>(),
    );

    let indicator_light = SpotLight {
        color: Srgba::hex("#20bf04").unwrap().into(),
        intensity: 13_000.,
        range: 1.3,

        ..Default::default()
    };

    commands.spawn_batch(
        (0..60)
            .map(|indicator_index| {
                let mut transform = Transform::from_translation(Vec3::new(0., 5., 0.25));

                transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_rotation_z(TAU / 60. * (indicator_index as f32)),
                );

                transform = transform.looking_at(Vec3::ZERO, Dir3::Y);

                (indicator_light.clone(), transform)
            })
            .collect::<Vec<_>>(),
    );

    let hour_minute_arm_material = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#000000").unwrap().into(),
        perceptual_roughness: 1. - 0.213,

        ..Default::default()
    }));

    let arms_config = vec![
        (
            Transform::from_translation(Vec3::new(0., 0.75, 0.25))
                .with_scale(Vec3::new(0.3, 2.5, 0.1)),
            hour_minute_arm_material.clone(),
            TimeComponentType::Hour,
            Timer::from_seconds(2.5, TimerMode::Once),
        ),
        (
            Transform::from_translation(Vec3::new(0., 1., 0.35))
                .with_scale(Vec3::new(0.2, 4., 0.1)),
            hour_minute_arm_material.clone(),
            TimeComponentType::Minute,
            Timer::from_seconds(1.5, TimerMode::Once),
        ),
        (
            Transform::from_translation(Vec3::new(0., 1.25, 0.45))
                .with_scale(Vec3::new(0.1, 5., 0.1)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Srgba::hex("#B30000").unwrap().into(),
                perceptual_roughness: 1. - 0.213,

                ..Default::default()
            })),
            TimeComponentType::Second,
            Timer::from_seconds(0.5, TimerMode::Once),
        ),
    ];

    for (transform, material, arm, timer) in arms_config {
        commands.spawn((
            Transform::default(),
            arm,
            AnimationEasingCurve(EasingCurve::new(0., 0., EaseFunction::Linear)),
            AnimationTimer(timer),
            Visibility::Visible,
            children![(unit_cube_mesh.clone(), material, transform)],
        ));
    }

    commands.spawn(DirectionalLight {
        color: Color::WHITE,
        illuminance: 6000.,
        shadow_maps_enabled: true,

        ..Default::default()
    });

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0., 0., 10.)),
        Projection::Perspective(PerspectiveProjection {
            fov: 60_f32.to_radians(),
            near: 0.3,
            far: 1000.,
            ..Default::default()
        }),
        Tonemapping::None,
    ));
}

fn time_component_to_radians(
    time_component_value: u8,
    time_component_type: &TimeComponentType,
    radians: f32,
) -> f32 {
    let in_end = match time_component_type {
        TimeComponentType::Hour => 24_f32,
        TimeComponentType::Minute => 60_f32,
        TimeComponentType::Second => 60_f32,
    };

    (time_component_value as f32).remap(0., in_end, 0., radians)
}

fn set_animation_timers(
    mut previous_time: ResMut<PreviousTime>,
    mut run_animations: ResMut<RunAnimations>,
    mut query: Query<(
        &mut AnimationEasingCurve,
        &mut AnimationTimer,
        &TimeComponentType,
    )>,
) {
    let now = OffsetDateTime::now_local()
        .expect("Should always be able to get local time")
        .time()
        .truncate_to_second();

    let now_option = Some(now);

    let previous_time_option = previous_time.0;

    if previous_time_option == now_option {
        return;
    }

    let (hour, minute, second) = (now.hour(), now.minute(), now.second());

    let (previous_hour_option, previous_minute_option, previous_second_option) =
        match previous_time_option {
            Some(previous_time_value) => (
                Some(previous_time_value.hour()),
                Some(previous_time_value.minute()),
                Some(previous_time_value.second()),
            ),
            None => (None, None, None),
        };

    query.par_iter_mut().for_each(
        |(mut animation_easing_curve, mut animation_timer, time_component_type)| {
            let end_option = match time_component_type {
                TimeComponentType::Hour => {
                    if previous_hour_option == Some(hour) {
                        None
                    } else {
                        Some(time_component_to_radians(
                            hour,
                            time_component_type,
                            -TAU * 2.,
                        ))
                    }
                }

                TimeComponentType::Minute => {
                    if previous_minute_option == Some(minute) {
                        None
                    } else {
                        Some(time_component_to_radians(minute, time_component_type, -TAU))
                    }
                }

                TimeComponentType::Second => {
                    if previous_second_option == Some(second) {
                        None
                    } else {
                        Some(time_component_to_radians(second, time_component_type, -TAU))
                    }
                }
            };

            if let Some(end) = end_option {
                let easing_curve = &animation_easing_curve.0;

                let start = match previous_time_option {
                    Some(_) => easing_curve.sample_clamped(1.),
                    None => end,
                };

                animation_easing_curve.0 = EasingCurve::new(start, end, EaseFunction::CubicInOut);

                animation_timer.0.reset();
            }
        },
    );

    previous_time.0 = now_option;

    run_animations.0 = true;
}

fn animate_arms(
    mut query: Query<(&mut Transform, &mut AnimationTimer, &AnimationEasingCurve)>,
    mut run_animations: ResMut<RunAnimations>,
    time: Res<Time>,
) {
    let some_timers_are_active = AtomicBool::new(false);

    query.par_iter_mut().for_each(
        |(mut transform, mut animation_timer, animation_easing_curve)| {
            let timer = &mut animation_timer.0;

            if timer.is_finished() {
                return;
            }

            let easing_curve = &animation_easing_curve.0;

            if easing_curve.sample_clamped(0.) == easing_curve.sample_clamped(1.) {
                timer.finish();
            }

            let fraction = timer
                .tick(Duration::from_secs_f32(time.delta_secs()))
                .fraction();

            let angle = easing_curve.sample_clamped(fraction);

            transform.rotation = Quat::from_rotation_z(angle);

            if !timer.is_finished() {
                some_timers_are_active.store(true, Ordering::Relaxed);
            }
        },
    );

    if !some_timers_are_active.load(Ordering::Relaxed) {
        run_animations.0 = false;
    }
}
