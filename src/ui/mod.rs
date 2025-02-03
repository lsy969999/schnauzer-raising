use bevy::prelude::*;

use crate::{app::state::AppState, asset::resource::DefaultAsset};
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        //
        app.add_systems(OnEnter(AppState::Home), test_image);
        app.add_systems(Update, update_image.run_if(in_state(AppState::Home)));
        // app.add_systems(OnEnter(AppState::Home2), test_image2);
    }
}

fn test_image(mut commands: Commands, def_asset: Res<DefaultAsset>) {
    commands.spawn(Text::new("home"));
    commands.spawn(Sprite::from_image(def_asset.bevy_image.clone()));
}

fn update_image(mut q: Query<(&Sprite, &mut Transform)>, time: Res<Time>) {
    for (_, mut tr) in &mut q {
        let x = time.elapsed_secs().sin() * 100.;
        tr.translation.x = x;
    }
}

fn test_image2(mut commands: Commands, def_asset: Res<DefaultAsset>) {
    commands.spawn(Text::new("home2"));
    commands.spawn(Sprite::from_image(def_asset.bevy_image.clone()));
}
