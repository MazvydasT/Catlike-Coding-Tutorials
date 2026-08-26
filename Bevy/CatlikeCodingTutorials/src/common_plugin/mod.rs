use bevy::{
    DefaultPlugins,
    app::{App, Plugin, PluginGroup},
    light::GlobalAmbientLight,
    utils::default,
    window::{PresentMode, Window, WindowPlugin},
};

mod toggleable_diagnostics_plugin;
use toggleable_diagnostics_plugin::ToggleableDiagnosticsPlugin;

mod toggleable_world_inspector_plugin;
use toggleable_world_inspector_plugin::ToggleableWorldInspectorPlugin;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalAmbientLight::NONE)
            .add_plugins((DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),))
            .add_plugins((ToggleableWorldInspectorPlugin, ToggleableDiagnosticsPlugin));
    }
}
