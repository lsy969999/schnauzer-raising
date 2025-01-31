use bevy::prelude::*;

use crate::{app::state::AppState, asset::resource::DefaultAsset};
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        //
        app.add_systems(OnEnter(AppState::Home), test_image);
        app.add_systems(OnEnter(AppState::Home2), test_image2);
    }
}

fn test_image(mut commands: Commands, def_asset: Res<DefaultAsset>) {
    commands.spawn(Text::new("home"));
    commands.spawn(Sprite::from_image(def_asset.bevy_image.clone()));
}

fn test_image2(mut commands: Commands, def_asset: Res<DefaultAsset>) {
    commands.spawn(Text::new("home2"));
    commands.spawn(Sprite::from_image(def_asset.bevy_image.clone()));
}
