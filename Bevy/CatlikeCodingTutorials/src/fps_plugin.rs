use core::f64;
use std::{collections::VecDeque, time::Duration};

use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Handle},
    ecs::{
        children,
        component::Component,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        spawn::SpawnRelated,
        system::{Commands, Query, Res, ResMut},
    },
    text::{Font, TextFont, TextSpan},
    time::{Time, common_conditions::on_real_timer},
    ui::{Display, FlexDirection, Node, PositionType, Val, widget::Text},
};
use regex::Regex;

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

#[derive(Resource)]
struct DurationRegex {
    regex: Regex,
}

#[derive(Component)]
enum FPSTextType {
    Avg,
    Min,
}

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
        .insert_resource(DurationRegex {
            regex: Regex::new(r"^(\d+(?:\.\d+)?)([^0-9]+)$").unwrap(),
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font_size_resource: Res<FontSizeResource>,
) {
    let font_handle: Handle<Font> = asset_server.load("fonts/SpaceMono-Regular.ttf");

    let text_font = TextFont {
        font: font_handle.clone(),
        font_size: font_size_resource.font_size * 0.7,
        ..Default::default()
    };

    let fps_font = TextFont {
        font: font_handle.clone(),
        font_size: font_size_resource.font_size,
        ..Default::default()
    };

    let starting_string = format!("{: >15}", "");

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
                    TextSpan::new(&starting_string),
                    fps_font.clone(),
                    FPSTextType::Avg
                )]
            ),
            (
                Text::new("Min. FPS:"),
                text_font.clone(),
                children![(
                    TextSpan::new(&starting_string),
                    fps_font.clone(),
                    FPSTextType::Min
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
    duration_regex: Res<DurationRegex>,
    mut fps_text: Query<(&mut TextSpan, &FPSTextType)>,
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

        let avg_frame_duration_in_seconds = delta_time_sum / (queue.len() as f64);
        let avg_fps = 1. / avg_frame_duration_in_seconds;
        let min_fps = 1. / delta_time_max;

        fps_text
            .par_iter_mut()
            .for_each(|(mut text_span, fps_text_type)| {
                let (fps, duration_in_seconds) = match fps_text_type {
                    FPSTextType::Avg => (avg_fps, avg_frame_duration_in_seconds),
                    FPSTextType::Min => (min_fps, delta_time_max),
                };

                let duration_as_string: String = format!("{:?}", Duration::from_secs_f64(duration_in_seconds));
                let formatted_duration = if let Some(captures) = duration_regex.regex.captures(&duration_as_string) {
                    let numeric_part = captures
                        .get(1)
                        .map_or("", |m| m.as_str())
                        .parse::<f64>()
                        .unwrap_or(f64::NAN);
                    let units_part = captures.get(2).map_or("", |m| m.as_str());

                    format!("{: >6.2}{: <2}", numeric_part, units_part)
                } else {
                    String::from("ERROR")
                };

                text_span.0 = format!("{: >4.0} | {}", fps, formatted_duration);
            });
    }
}
