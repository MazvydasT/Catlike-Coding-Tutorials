use std::{collections::VecDeque, time::Duration};

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    text::{TextFont, TextSpan},
    time::{Time, common_conditions::on_real_timer},
    ui::{Node, PositionType, Val, widget::Text},
};

#[derive(Resource)]
struct DeltaTimeAccumulator {
    accumulator: VecDeque<f64>,
}

#[derive(Component)]
struct FPSText;

pub struct FPSPlugin;

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(DeltaTimeAccumulator {
            accumulator: VecDeque::with_capacity(100),
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                record_delta_time,
                show_stats.run_if(on_real_timer(Duration::from_secs(1))),
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Text::new("FPS:"),
            TextFont {
                font_size: 32.,
                ..Default::default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(16.),
                right: Val::Px(16.),
                ..Default::default()
            },
        ))
        .with_child((
            TextSpan::new("   "),
            TextFont {
                font_size: 32.,
                ..Default::default()
            },
            FPSText,
        ));
}

fn record_delta_time(time: Res<Time>, mut delta_time_accumulator: ResMut<DeltaTimeAccumulator>) {
    let delta = time.delta_secs_f64();

    let accumulator = &mut delta_time_accumulator.accumulator;

    if accumulator.len() == accumulator.capacity() {
        accumulator.pop_front();
    }

    accumulator.push_back(delta);
}

fn show_stats(
    delta_time_accumulator: Res<DeltaTimeAccumulator>,
    mut query: Query<&mut TextSpan, With<FPSText>>,
) {
    let accumulator = &delta_time_accumulator.accumulator;

    let sum = accumulator.iter().sum::<f64>();
    let average_fps = (1. / (sum / (accumulator.len() as f64))).round();

    for mut span in &mut query {
        **span = format!("{: >3}", average_fps);
    }
}
