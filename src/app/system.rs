use bevy::prelude::*;

use super::{component::My2DCamera, state::AppState};

pub(super) fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, My2DCamera));
}

pub(super) fn version_check(mut next_app_state: ResMut<NextState<AppState>>) {
    info!("version_check");
    // app version
    // asset version
    next_app_state.set(AppState::DefaultAssetLoading);
}
