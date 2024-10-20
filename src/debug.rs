use bevy::prelude::*;

use crate::setting::SystemSettings;

pub fn debug_plugin(app: &mut App) {
    app.add_systems(Update, debug_draw);
}

fn debug_draw(settings: ResMut<SystemSettings>, transforms: Query<&Transform>, mut gizmo: Gizmos) {
    if settings.draw_transform {
        for transform in transforms.iter() {
            gizmo.axes(*transform, 40.0);
        }
    }
}
