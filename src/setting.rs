use bevy::{
    input::keyboard::KeyboardInput,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_egui::{egui, EguiContexts};

use crate::languages::{LangDict, CHINESE, ENGLISH};

#[derive(Resource)]
pub struct SystemSettings {
    pub controlling_camera: bool,
    pub draw_transform: bool,
    pub lang_dict: &'static LangDict,
}

pub fn setting_plugin(app: &mut App) {
    app.add_systems(Startup, setting_setup);
    app.add_systems(Update, setting_update);
}

fn setting_setup(mut commands: Commands) {
    commands.insert_resource(SystemSettings {
        controlling_camera: false,
        draw_transform: false,
        lang_dict: &ENGLISH,
    });
}

fn setting_update(
    mut contexts: EguiContexts,
    settings: ResMut<SystemSettings>,
    mut main_window: Query<&mut Window, With<PrimaryWindow>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    cameras: Query<&Transform, With<Camera>>,
) {
    let settings = settings.into_inner();
    let lang_dict = settings.lang_dict;
    egui::Window::new(lang_dict.settings).show(contexts.ctx_mut(), |ui| {
        ui.collapsing(lang_dict.debug, |ui| {
            ui.checkbox(&mut settings.draw_transform, lang_dict.draw_transforms);
            let camera = cameras.get_single().unwrap();
            ui.label(format!("{:?}", camera))
        });
        ui.collapsing(lang_dict.camera, |ui| {
            if ui.button(lang_dict.control_camera_style1).clicked() {
                let mut window = main_window.get_single_mut().unwrap();
                window.cursor.visible = false;
                window.cursor.grab_mode = CursorGrabMode::Confined;
                settings.controlling_camera = true;
            }
        });
        egui::ComboBox::from_id_salt("Language")
            .selected_text("Language")
            .show_ui(ui, |ui| {
                if ui.button("English").clicked() {
                    settings.lang_dict = &ENGLISH;
                }
                if ui.button("中文").clicked() {
                    settings.lang_dict = &CHINESE;
                }
            });
    });
    for event in keyboard_input_events.read() {
        if event.key_code == KeyCode::Escape {
            if settings.controlling_camera {
                settings.controlling_camera = false;
                let mut window = main_window.get_mut(event.window).unwrap();
                window.cursor.visible = true;
                window.cursor.grab_mode = CursorGrabMode::None;
            }
        }
    }
}
