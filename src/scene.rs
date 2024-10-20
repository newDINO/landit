use avian3d::prelude::*;
use bevy::{color::palettes, prelude::*};

pub fn scene_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            color: Color::from(palettes::tailwind::AMBER_50),
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::IDENTITY.looking_to(Vec3::new(-1.0, -1.0, -1.0), Vec3::Y),
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
