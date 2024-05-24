use bevy::prelude::*;
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    let translation = Vec3::new(2.0, 3.0, 2.0);
    let focus = Vec3::new(0.0, 0.5, 0.0);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 7500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(translation).looking_at(focus, Vec3::Y),
        ..default()
    });
}
