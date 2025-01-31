use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct DefaultAsset {
    #[asset(path = "image/bevy_icon.png")]
    // #[asset(path = "https://s3.johanhelsing.studio/dump/favicon.png")]
    // #[asset(path = "http://127.0.0.1:5500/assets/image/bevy_icon.png")]
    pub bevy_image: Handle<Image>,
}
