use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use resource::DefaultAsset;

use crate::app::state::AppState;

pub mod resource;
#[cfg(any(target_os = "android", target_os = "ios"))]
mod source_plugin;
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::DefaultAssetLoading)
                .continue_to_state(AppState::Home)
                .load_collection::<DefaultAsset>(),
        );

        #[cfg(any(target_os = "android", target_os = "ios"))]
        {
            // app.add_plugins(source_plugin::FilesAssetPlugin);
        }
    }
}
