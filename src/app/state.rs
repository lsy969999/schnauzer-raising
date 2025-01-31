use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, States, Default)]
pub enum AppState {
    #[default]
    VersionCheck,
    DefaultAssetLoading,
    Home,
    Home2,
}
