use bevy::prelude::*;
use bevy_inspector_egui::{
    egui::{self, Align2},
    DefaultInspectorConfigPlugin,
};
use iyes_perf_ui::{prelude::*, PerfUiPlugin};

use crate::{app::state::AppState, asset::resource::DefaultAsset};

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin)
            .add_plugins(DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
            .add_systems(Update, inspector_ui);

        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, perf_ui);
    }
}

fn inspector_ui(world: &mut World, mut is_perf_ui_hide: Local<bool>) {
    let Ok(egui_context) = world
        .query_filtered::<&mut bevy_inspector_egui::bevy_egui::EguiContext, With<bevy::window::PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI")
        .pivot(Align2::LEFT_CENTER)
        .default_open(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // equivalent to `WorldInspectorPlugin`
                bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

                // egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                //     bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
                // });

                // ui.heading("Entities");
                // bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);

                if ui.button("perf toggle").clicked() {
                    let mut q = world.query::<(&PerfUiRoot, &mut Visibility)>();
                    let (_, mut v) = q.single_mut(world);
                    if *is_perf_ui_hide {
                        *v = Visibility::Visible;
                    } else {
                        *v = Visibility::Hidden;
                    }
                    *is_perf_ui_hide = !*is_perf_ui_hide;
                }

                if ui.button("test home toggle").clicked() {
                    let mut a = world.query_filtered::<Entity, With<Sprite>>();
                    let v = a.iter(world).collect::<Vec<_>>();
                    for b in v {
                        world.commands().entity(b).despawn_recursive();
                    }

                    world.commands().remove_resource::<DefaultAsset>();
                    let current_state = world.resource::<State<AppState>>().get().clone();

                    let mut next_state = world.resource_mut::<NextState<AppState>>();
                    match current_state {
                        AppState::Home => {
                            next_state.set(AppState::DefaultAssetLoading);
                        }
                        _ => {
                            // next_state.set(AppState::Home);
                        }
                    }
                }
            });
        });
}

fn perf_ui(mut commands: Commands) {
    commands
        .spawn(PerfUiRoot::default())
        .insert(PerfUiEntryFPS::default())
        .insert(PerfUiEntryEntityCount::default())
        .insert(PerfUiEntryCursorPosition::default())
        .insert(PerfUiEntryFrameCount::default())
        .insert(PerfUiEntryRunningTime::default())
        .insert(PerfUiEntryWindowResolution::default())
        .insert(PickingBehavior::IGNORE)
        .insert(Name::new("PerfUi"));
}
