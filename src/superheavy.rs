use avian3d::prelude::*;
use bevy::prelude::*;

pub fn super_heavy_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, apply_engine_force);
    app.add_systems(Update, draw_flame);
}

#[derive(Component)]
pub struct SuperHeavy {
    pub angle: f32,
    pub thrust: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let super_heavy_r = 4.5;
    let super_heavy_h = 71.0;
    commands.spawn((
        RigidBody::Dynamic,
        SuperHeavy {
            angle: 0.0,
            thrust: 1.0,
        },
        ExternalForce::ZERO.with_persistence(false),
        ColliderDensity(100.0),
        Collider::compound(vec![
            (
                Vec3::ZERO,
                Quat::IDENTITY,
                Collider::cylinder(super_heavy_r, super_heavy_h),
            ),
            (
                Vec3::new(0.0, super_heavy_h * 0.3, -super_heavy_r),
                Quat::IDENTITY,
                Collider::cuboid(0.5, 0.5, 1.5),
            ),
            (
                Vec3::new(0.0, super_heavy_h * 0.3, super_heavy_r),
                Quat::IDENTITY,
                Collider::cuboid(0.5, 0.5, 1.5),
            ),
        ]),
        // LockedAxes::new()
        //     .lock_rotation_x()
        //     .lock_rotation_y()
        //     .lock_translation_z(),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(super_heavy_r, super_heavy_h)),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                metallic: 0.99,
                reflectance: 0.7,
                ..Default::default()
            }),
            ..Default::default()
        },
    ));
}

fn draw_flame(
    transform: Query<&Transform, With<SuperHeavy>>,
    super_heavy: Query<&SuperHeavy>,
    mut gizmo: Gizmos,
) {
    let super_heavy = super_heavy.get_single().unwrap();
    let transform = *transform.get_single().unwrap();

    let start = Vec3::new(0.0, -35.0, 0.0);
    let length = super_heavy.thrust * 100.0;
    let endx = super_heavy.angle.sin() * length;
    let endy = start.y - super_heavy.angle.cos() * length;
    gizmo.arrow(
        transform * start,
        transform * Vec3::new(endx, endy, 0.0),
        Color::WHITE,
    );
}

fn apply_engine_force(
    mut force: Query<&mut ExternalForce, With<SuperHeavy>>,
    transform: Query<&Transform, With<SuperHeavy>>,
    super_heavy: Query<&SuperHeavy>,
) {
    let super_heavy = super_heavy.get_single().unwrap();
    let transform = *transform.get_single().unwrap();
    let mut force = force.get_single_mut().unwrap();

    let point = transform * Vec3::new(0.0, -35.0, 0.0);
    let strength = super_heavy.thrust * 3.0 * 2.26e6;
    let fy = strength * super_heavy.angle.cos();
    let fx = -strength * super_heavy.angle.sin();
    let f = transform.rotation * Vec3::new(fx, fy, 0.0);

    force.apply_force_at_point(f, point, transform * Vec3::ZERO);
}
