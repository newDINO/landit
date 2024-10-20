use bevy_egui::EguiPlugin;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use avian3d::prelude::*;
use bevy::prelude::*;

mod camera;
mod debug;
mod fonts;
mod languages;
mod mechazilla;
mod scene;
mod setting;
mod superheavy;
mod ui;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    name: Some("Land it!".to_owned()),
                    canvas: Some("#myCanvas".to_owned()),
                    fit_canvas_to_parent: true,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            PhysicsDebugPlugin::default(),
            PhysicsPlugins::default(),
            EguiPlugin,
            ui::ui_plugin,
            superheavy::super_heavy_plugin,
            scene::scene_plugin,
            setting::setting_plugin,
            debug::debug_plugin,
            mechazilla::mechazilla_plugin,
            fonts::font_plugin,
            camera::CameraControlPlugin {
                camera_bundle: Camera3dBundle {
                    camera: Camera {
                        // clear_color: ClearColorConfig::Custom(Color::from(bevy::color::palettes::tailwind::BLUE_400)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(00.0, 100.0, 500.0)
                        .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
                    projection: Projection::Perspective(PerspectiveProjection {
                        near: 0.01,
                        far: 10000.0,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                speed: 100.0,
                sensitivity: 0.3,
            },
        ))
        .run();
}
