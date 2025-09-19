use core::f64;
use std::{collections::VecDeque, time::Duration};

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        children,
        component::Component,
        query::{With, Without},
        resource::Resource,
        schedule::IntoScheduleConfigs,
        spawn::SpawnRelated,
        system::{Commands, Res, ResMut, Single},
    },
    text::{TextFont, TextSpan},
    time::{Time, common_conditions::on_real_timer},
    ui::{Display, FlexDirection, Node, PositionType, Val, widget::Text},
};

pub struct FPSPlugin {
    pub font_size: f32,
    pub history_size: usize,
    pub refresh_ui_every: Duration,
}

#[derive(Resource)]
struct DeltaTimeQueueResource {
    queue: VecDeque<f64>,
}

#[derive(Resource)]
struct FontSizeResource {
    font_size: f32,
}

#[derive(Component)]
struct AvgFPSText;

#[derive(Component)]
struct MinFPSText;

impl Default for FPSPlugin {
    fn default() -> Self {
        Self {
            font_size: 32.,
            history_size: 100,
            refresh_ui_every: Duration::from_secs(1),
        }
    }
}

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(DeltaTimeQueueResource {
            queue: VecDeque::with_capacity(self.history_size),
        })
        .insert_resource(FontSizeResource {
            font_size: self.font_size,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                record_delta_time,
                show_stats.run_if(on_real_timer(self.refresh_ui_every)),
            ),
        );
    }
}

fn setup(mut commands: Commands, font_size_resource: Res<FontSizeResource>) {
    let text_font = TextFont {
        font_size: font_size_resource.font_size * 0.7,
        ..Default::default()
    };

    let fps_font = TextFont {
        font_size: font_size_resource.font_size,
        ..Default::default()
    };

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.),
            right: Val::Px(16.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![
            (
                Text::new("Avg. FPS:"),
                text_font.clone(),
                children![(
                    TextSpan::new("    "),
                    fps_font.clone(),
                    AvgFPSText,
                )]
            ),
            (
                Text::new("Min. FPS:"),
                text_font.clone(),
                children![(
                    TextSpan::new("    "),
                    fps_font.clone(),
                    MinFPSText,
                )]
            )
        ],
    ));
}

fn record_delta_time(time: Res<Time>, mut delta_time_accumulator: ResMut<DeltaTimeQueueResource>) {
    let delta = time.delta_secs_f64();

    let accumulator = &mut delta_time_accumulator.queue;

    if accumulator.len() == accumulator.capacity() {
        accumulator.pop_front();
    }

    accumulator.push_back(delta);
}

fn show_stats(
    delta_time_accumulator: Res<DeltaTimeQueueResource>,
    mut avg_fps_text: Single<&mut TextSpan, (With<AvgFPSText>, Without<MinFPSText>)>,
    mut min_fps_text: Single<&mut TextSpan, (With<MinFPSText>, Without<AvgFPSText>)>,
) {
    let queue = &delta_time_accumulator.queue;

    let queue_len = queue.len();

    if queue_len > 0 {
        let mut delta_time_max = 0.;
        let mut delta_time_sum = 0.;

        for delta_time in queue.iter() {
            delta_time_max = f64::max(delta_time_max, *delta_time);
            delta_time_sum += delta_time;
        }

        let avg_fps = 1. / (delta_time_sum / (queue.len() as f64));
        let min_fps = 1. / delta_time_max;

        avg_fps_text.0 = format!("{: >4.0}", avg_fps);
        min_fps_text.0 = format!("{: >4.0}", min_fps);
    }
}
