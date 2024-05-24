use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin);
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    let translation = Vec3::new(0.0, 0.5, 2.0);
    let focus = Vec3::new(0.0, 0.5, 0.0);
    let radius = Some((translation - focus).length());

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(focus, Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                near: 0.001,
                far: 5.0,
                ..default()
            }),
            ..default()
        },
        PanOrbitCamera {
            focus,
            radius,
            button_pan: MouseButton::Right,
            button_orbit: MouseButton::Middle,
            orbit_smoothness: 0.0,
            pan_smoothness: 0.0,
            zoom_smoothness: 0.0,
            ..default()
        },
    ));
}
