use bevy::{app::App, camera::ClearColor, color::Color};
use catlike_coding_tutorials::CommonPlugin;

mod clock_plugin;
use clock_plugin::ClockPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((CommonPlugin, ClockPlugin))
        .run();
}
