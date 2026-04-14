use std::time::Duration;

use bevy::{
    DefaultPlugins,
    app::{App, Plugin, PluginGroup},
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    utils::default,
    window::{PresentMode, Window, WindowPlugin},
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins.set(WindowPlugin {
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
            ));
    }
}
