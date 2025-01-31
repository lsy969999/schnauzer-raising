use bevy::prelude::*;

use super::{FfiApp, FfiAppInterface};

impl FfiAppInterface for FfiApp {
    fn main() {
        App::new()
            .add_plugins(DefaultPlugins.set(AssetPlugin {
                file_path: format!("assets"),
                ..default()
            }))
            .add_plugins(crate::app::AppPlugin)
            .run();
    }
}
