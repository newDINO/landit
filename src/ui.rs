use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::superheavy::SuperHeavy;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Update, dashboard_update);
    app.add_systems(Update, control_thrust);
    app.add_systems(Update, control_angle);
}

pub fn dashboard_update(
    super_heavy: Query<(&LinearVelocity, &AngularVelocity), With<SuperHeavy>>,
    mut contexts: EguiContexts,
) {
    let (vel, angular) = if let Ok(vel) = super_heavy.get_single() {
        vel
    } else {
        return;
    };
    egui::Window::new("DashBoard").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Velocity: x: {:.3}, y: {:.3}", vel.x, vel.y));
        ui.label(format!("Angular velocity: {:.7}", angular.z));
    });
}

pub fn control_thrust(mut super_heavy: Query<&mut SuperHeavy>, mut contexts: EguiContexts) {
    let super_heavy = super_heavy.get_single_mut().unwrap().into_inner();
    egui::Window::new("Thrust").show(contexts.ctx_mut(), |ui| {
        let slider = egui::Slider::new(&mut super_heavy.thrust, 0.4..=1.0);
        ui.add(slider);
    });
}

pub fn control_angle(mut super_heavy: Query<&mut SuperHeavy>, mut contexts: EguiContexts) {
    let super_heavy = super_heavy.get_single_mut().unwrap().into_inner();
    egui::Window::new("Angle").show(contexts.ctx_mut(), |ui| {
        let slider = egui::Slider::new(&mut super_heavy.angle, -0.3..=0.3);
        ui.add(slider);
    });
}
