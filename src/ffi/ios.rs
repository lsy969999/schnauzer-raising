use bevy::prelude::*;

use super::{FfiApp, FfiAppInterface};

impl FfiAppInterface for FfiApp {
    fn main() {
        App::new().add_plugins(DefaultPlugins).run();
    }
}
