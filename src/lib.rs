use bevy_egui::EguiPlugin;
use superheavy::super_heavy_plugin;
use ui::ui_plugin;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use avian3d::prelude::*;
use bevy::{color::palettes, prelude::*};

mod superheavy;
mod ui;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            EguiPlugin,
            ui_plugin,
            super_heavy_plugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, debug_draw)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    commands.spawn(Camera3dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::from(palettes::tailwind::BLUE_400)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 100.0, 500.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection {
            near: 0.01,
            far: 10000.0,
            ..Default::default()
        }),
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            color: Color::from(palettes::tailwind::AMBER_50),
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::IDENTITY.looking_to(Vec3::new(-1.0,-1.0, -1.0), Vec3::Y),
        ..Default::default()
    });
    ambient_light.brightness = 100.0;

    let ground_a = 10000.0;
    let ground_h = 1.0;
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(ground_a, ground_h, ground_a),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(ground_a, ground_h, ground_a)),
            material: materials.add(StandardMaterial {
                base_color: Color::from(palettes::css::IVORY),
                ..Default::default()
            }),
            ..Default::default()
        },
    ));
}

fn debug_draw(transforms: Query<&Transform>, mut gizmo: Gizmos) {
    for transform in transforms.iter() {
        gizmo.axes(*transform, 40.0);
    }
}
