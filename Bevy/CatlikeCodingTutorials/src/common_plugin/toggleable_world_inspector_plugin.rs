use bevy::{
    app::Plugin,
    input::{common_conditions::input_toggle_active, keyboard::KeyCode},
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct ToggleableWorldInspectorPlugin;

impl Plugin for ToggleableWorldInspectorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::KeyI)),
        ));
    }
}
