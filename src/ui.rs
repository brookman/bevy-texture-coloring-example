use bevy::{
    app::{App, Update},
    ecs::system::{ResMut, Resource},
};
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts,
};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>();
        app.add_systems(Update, update);
    }
}

#[derive(Resource)]
pub struct State {
    pub color: Color32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            color: Color32::from_rgb(255, 90, 72),
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update(mut contexts: EguiContexts, mut ui_state: ResMut<State>) {
    let egui_context = contexts.ctx_mut();

    let window = egui::Window::new("Config");

    window.show(egui_context, |ui| {
        ui.vertical(|ui| {
            ui.label("Customize color:");
            ui.color_edit_button_srgba(&mut ui_state.color);
        });
    });
}
