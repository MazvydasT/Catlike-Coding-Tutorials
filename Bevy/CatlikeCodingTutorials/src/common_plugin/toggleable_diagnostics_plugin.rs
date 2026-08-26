use bevy::{
    app::{Plugin, Startup, Update},
    camera::visibility::Visibility,
    dev_tools::diagnostics_overlay::{DiagnosticsOverlay, DiagnosticsOverlayPlugin},
    diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::{
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Single},
    },
    input::{common_conditions::input_just_pressed, keyboard::KeyCode},
};

pub struct ToggleableDiagnosticsPlugin;

impl Plugin for ToggleableDiagnosticsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            DiagnosticsOverlayPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle.run_if(input_just_pressed(KeyCode::KeyF)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((DiagnosticsOverlay::fps(), Visibility::Hidden));
}

fn toggle(mut visibility: Single<&mut Visibility, With<DiagnosticsOverlay>>) {
    visibility.toggle_visible_hidden();
}
