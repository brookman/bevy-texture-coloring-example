use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod light;
mod soldier;
mod ui;

static APP_NAME: &str = "Bevy Texture Coloring Example";

fn main() {
    let mut app = App::new();

    // Default plugins -------------------------------------

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: APP_NAME.to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };
    let log_plugin = bevy::log::LogPlugin {
        level: bevy::log::Level::INFO,
        ..default()
    };
    let asset_plugin = AssetPlugin {
        watch_for_changes_override: Some(true),
        ..Default::default()
    };
    let default_plugins = DefaultPlugins
        .set(window_plugin)
        .set(log_plugin)
        .set(asset_plugin);

    // -----------------------------------------------------

    app.add_plugins((
        default_plugins,
        WorldInspectorPlugin::new(),
        ui::Plugin,
        camera::Plugin,
        light::Plugin,
        soldier::Plugin,
    ));

    app.run();
}
