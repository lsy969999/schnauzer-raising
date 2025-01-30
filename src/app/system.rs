use bevy::prelude::*;

use super::component::My2DCamera;

pub(super) fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, My2DCamera));
}
