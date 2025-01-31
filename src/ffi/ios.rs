use bevy::prelude::*;

use super::{FfiApp, FfiAppInterface};

impl FfiAppInterface for FfiApp {
    fn main() {
        App::new()
            .add_plugins(
                DefaultPlugins
                    .set(bevy::render::RenderPlugin {
                        render_creation: bevy::render::settings::RenderCreation::Automatic(
                            bevy::render::settings::WgpuSettings {
                                // backends: Some(bevy::render::settings::Backends::METAL),
                                ..default()
                            },
                        ),
                        ..default()
                    })
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            resizable: false,
                            mode: bevy::window::WindowMode::BorderlessFullscreen(
                                MonitorSelection::Primary,
                            ),
                            // resolution: WindowResolution::default().with_scale_factor_override(5.),
                            ..default()
                        }),
                        ..default()
                    }),
            )
            .add_plugins(crate::app::AppPlugin)
            .insert_resource(bevy::winit::WinitSettings::mobile())
            .run();
    }
}
