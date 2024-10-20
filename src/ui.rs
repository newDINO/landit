use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{mechazilla::Mechazilla, setting::SystemSettings, superheavy::SuperHeavy};

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Update, (control_panel, dashboard_update));
}

fn dashboard_update(
    super_heavy: Query<(&LinearVelocity, &AngularVelocity), With<SuperHeavy>>,
    settings: Res<SystemSettings>,
    mut contexts: EguiContexts,
) {
    let (vel, angular) = if let Ok(vel) = super_heavy.get_single() {
        vel
    } else {
        return;
    };
    let lang_dict = settings.lang_dict;
    egui::Window::new(lang_dict.dashboard)
        .default_pos((10000.0, 100.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.label(format!("Velocity: x: {:.3}, y: {:.3}", vel.x, vel.y));
            ui.label(format!("Angular velocity: {:.7}", angular.z));
        });
}

fn control_panel(
    settings: Res<SystemSettings>,
    mut contexts: EguiContexts,
    mut super_heavy: Query<&mut SuperHeavy>,
    mechazilla: ResMut<Mechazilla>,
    mut forces: Query<&mut ExternalForce>,
    mut prismatic_joints: Query<&mut PrismaticJoint>,
    transforms: Query<&Transform>,
) {
    let lang_dict = settings.lang_dict;
    let super_heavy = super_heavy.get_single_mut().unwrap().into_inner();
    let mechazilla = mechazilla.into_inner();
    egui::Window::new("Controls").show(contexts.ctx_mut(), |ui| {
        ui.label("SuperHeavy");

        ui.label(lang_dict.thrust);
        let slider = egui::Slider::new(&mut super_heavy.thrust, 0.4..=1.0);
        ui.add(slider);

        ui.label(lang_dict.angle);
        let slider = egui::Slider::new(&mut super_heavy.angle, -0.3..=0.3);
        ui.add(slider);

        ui.separator();
        ui.label("Mechazilla");
        let mut slider_force = forces.get_mut(mechazilla.slider).unwrap();
        ui.label(lang_dict.chopstick_up_down);
        let slider = egui::Slider::new(&mut slider_force.y, -3e4..=3e4);
        ui.add(slider);

        let mut slider_joint = prismatic_joints.get_mut(mechazilla.slider_joint).unwrap();
        ui.checkbox(&mut mechazilla.fix_up_down, "Fix Up Down");
        let slider_transform = transforms.get(mechazilla.slider).unwrap();
        let tower_transform = transforms.get(mechazilla.tower).unwrap();
        if mechazilla.fix_up_down {
            let slider_coord = slider_transform.translation.y - tower_transform.translation.y;
            slider_joint.free_axis_limits = Some(DistanceLimit::new(slider_coord, slider_coord));
        } else {
            slider_joint.free_axis_limits = Some(DistanceLimit::new(
                mechazilla.slider_limit.0,
                mechazilla.slider_limit.1,
            ));
        }
    });
}
