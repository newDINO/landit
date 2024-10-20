use avian3d::prelude::*;
use bevy::{color::palettes, prelude::*};

pub fn mechazilla_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(PostProcessCollisions, filter_collision);
}

#[derive(Resource)]
pub struct Mechazilla {
    pub tower: Entity,
    pub slider: Entity,
    pub arm1: Entity,
    pub arm2: Entity,

    pub slider_joint: Entity,
    pub fix_up_down: bool,
    pub slider_limit: (f32, f32),
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tower_size = Vec3::new(11.0, 130.0, 11.0);
    let tower_transform = Transform::from_xyz(20.0, tower_size.y * 0.5, 0.0);
    let tower = commands
        .spawn((
            RigidBody::Static,
            Collider::cuboid(tower_size.x, tower_size.y, tower_size.z),
            PbrBundle {
                mesh: meshes.add(Cuboid::from_size(tower_size)),
                material: materials.add(Color::from(palettes::tailwind::GRAY_700)),
                transform: tower_transform,
                ..Default::default()
            },
        ))
        .id();

    let slider_size = Vec3::new(13.0, 7.0, 13.0);
    let slider_transform = tower_transform * Transform::from_xyz(0.0, 30.0, 0.0);
    let slider = commands
        .spawn((
            RigidBody::Dynamic,
            ExternalForce::ZERO,
            Collider::cuboid(slider_size.x, slider_size.y, slider_size.z),
            PbrBundle {
                mesh: meshes.add(Cuboid::from_size(slider_size)),
                material: materials.add(Color::from(palettes::tailwind::GRAY_700)),
                transform: slider_transform,
                ..Default::default()
            },
        ))
        .id();
    let slider_limit = (0.0, 50.0);
    let slider_joint = commands
        .spawn(
            PrismaticJoint::new(tower, slider)
                .with_free_axis(Vec3::Y)
                .with_limits(slider_limit.0, slider_limit.1)
                .with_linear_velocity_damping(1.0),
        )
        .id();

    let upper_arm_size = Vec3::new(7.0, slider_size.y, 0.5);
    let angle1 = std::f32::consts::FRAC_PI_4;
    let lower_arm_size = Vec3::new(20.0, 5.0, 0.5);

    let arm_transform = slider_transform * Transform::from_xyz(-slider_size.x * 0.5, 0.0, 0.0);
    let arm1 = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::compound(vec![
                (
                    Vec3::new(
                        -upper_arm_size.x * 0.5 * angle1.cos(),
                        0.0,
                        upper_arm_size.x * 0.5 * angle1.sin(),
                    ),
                    Quat::from_axis_angle(Vec3::Y, angle1),
                    Collider::cuboid(upper_arm_size.x, upper_arm_size.y, upper_arm_size.z),
                ),
                (
                    Vec3::new(
                        -upper_arm_size.x * angle1.cos() - lower_arm_size.x * 0.5,
                        upper_arm_size.y * 0.25,
                        upper_arm_size.x * angle1.sin(),
                    ),
                    Quat::IDENTITY,
                    Collider::cuboid(lower_arm_size.x, lower_arm_size.y, lower_arm_size.z),
                ),
            ]),
            TransformBundle {
                local: arm_transform,
                ..Default::default()
            },
        ))
        .id();
    commands.spawn(
        RevoluteJoint::new(slider, arm1)
            .with_aligned_axis(Vec3::Y)
            .with_local_anchor_1(Vec3::new(-slider_size.x * 0.5, 0.0, 0.0))
            .with_angle_limits(0.0, 0.0001),
    );

    let angle2 = -angle1;
    let arm2 = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::compound(vec![
                (
                    Vec3::new(
                        -upper_arm_size.x * 0.5 * angle2.cos(),
                        0.0,
                        upper_arm_size.x * 0.5 * angle2.sin(),
                    ),
                    Quat::from_axis_angle(Vec3::Y, angle2),
                    Collider::cuboid(upper_arm_size.x, upper_arm_size.y, upper_arm_size.z),
                ),
                (
                    Vec3::new(
                        -upper_arm_size.x * angle2.cos() - lower_arm_size.x * 0.5,
                        upper_arm_size.y * 0.25,
                        upper_arm_size.x * angle2.sin(),
                    ),
                    Quat::IDENTITY,
                    Collider::cuboid(lower_arm_size.x, lower_arm_size.y, lower_arm_size.z),
                ),
            ]),
            TransformBundle {
                local: arm_transform,
                ..Default::default()
            },
        ))
        .id();
    commands.spawn(
        RevoluteJoint::new(slider, arm2)
            .with_aligned_axis(Vec3::Y)
            .with_local_anchor_1(Vec3::new(-slider_size.x * 0.5, 0.0, 0.0))
            .with_angle_limits(0.0, 0.0001),
    );

    commands.insert_resource(Mechazilla {
        tower,
        slider,
        arm1,
        arm2,

        slider_joint,
        fix_up_down: true,
        slider_limit,
    });
}

fn filter_collision(mut collisions: ResMut<Collisions>, mechazilla: Res<Mechazilla>) {
    collisions.remove_collision_pair(mechazilla.tower, mechazilla.slider);
    collisions.remove_collision_pair(mechazilla.arm1, mechazilla.slider);
    collisions.remove_collision_pair(mechazilla.tower, mechazilla.arm1);
    collisions.remove_collision_pair(mechazilla.arm2, mechazilla.slider);
    collisions.remove_collision_pair(mechazilla.tower, mechazilla.arm2);
    collisions.remove_collision_pair(mechazilla.arm1, mechazilla.arm2);
}
