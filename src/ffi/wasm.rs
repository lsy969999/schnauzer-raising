use super::{FfiApp, FfiAppInterface};
use bevy::prelude::*;

impl FfiAppInterface for FfiApp {
    fn main() {
        App::new()
            .add_plugins(
                DefaultPlugins
                    .set(AssetPlugin {
                        meta_check: bevy::asset::AssetMetaCheck::Never,
                        ..default()
                    })
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            canvas: Some(String::from("#target")),
                            fit_canvas_to_parent: true,
                            ..default()
                        }),
                        ..default()
                    }),
            )
            .run();
    }
}
