use bevy::prelude::*;
use state::AppState;

pub mod component;
pub mod state;
pub mod system;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // external plugin

        // inner plugin
        app.add_plugins(crate::asset::AssetPlugin)
            .add_plugins(crate::ui::UiPlugin)
            .add_plugins(crate::asynctask::AsyncTaskPlugin);

        app.init_state::<AppState>();

        #[cfg(feature = "inspector")]
        {
            use super::inspector::InspectorPlugin;
            app.add_plugins(InspectorPlugin);
        }

        app.add_systems(Startup, system::setup_camera);
        app.add_systems(OnEnter(AppState::VersionCheck), system::version_check);
    }
}
