use std::{fs::File, io::Read, path::PathBuf};

use bevy::{
    prelude::*,
    window::{WindowResolution, ANDROID_APP},
};
use jni::{
    objects::{JObject, JString},
    JavaVM,
};

use super::{FfiApp, FfiAppInterface};

impl FfiAppInterface for FfiApp {
    fn main() {
        App::new()
            .add_plugins(
                DefaultPlugins
                    .set(bevy::render::RenderPlugin {
                        render_creation: bevy::render::settings::RenderCreation::Automatic(
                            bevy::render::settings::WgpuSettings {
                                backends: Some(bevy::render::settings::Backends::VULKAN),
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
                    })
                    .set(AssetPlugin {
                        file_path: format!("a"),
                        ..default()
                    }),
            )
            .add_plugins(crate::app::AppPlugin)
            .insert_resource(bevy::winit::WinitSettings::mobile())
            .add_systems(PostStartup, || {
                let z = get_files_path();

                let imgbyte = read_from_android("bevy_icon.png");

                info!("read from imgbyte: {}", imgbyte.len());
            })
            .run();
    }
}
fn get_files_path() -> String {
    let ctx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
    let mut env = vm.attach_current_thread().unwrap();

    let files_dir = env
        .call_method(ctx, "getFilesDir", "()Ljava/io/File;", &[])
        .expect("Failed to get filesDir")
        .l()
        .expect("Invalid object");

    let files_path: JString = env
        .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])
        .expect("Failed to get absolute path")
        .l()
        .expect("Invalid string")
        .into();
    let str: String = env.get_string(&files_path).unwrap().into();
    info!("get_files_path: {str:?}");
    str
}

fn tt() {
    // ANDROID_APP.get().unwrap().asset_manager();
    // let a = ANDROID_APP.get().unwrap();
}

fn read_from_android(file_name: &str) -> Vec<u8> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
    let mut env = vm.attach_current_thread().unwrap();

    let files_dir = env
        .call_method(ctx, "getFilesDir", "()Ljava/io/File;", &[])
        .expect("Failed to get filesDir")
        .l()
        .expect("Invalid object");

    let files_path: JString = env
        .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])
        .expect("Failed to get absolute path")
        .l()
        .expect("Invalid string")
        .into();
    let path_str: String = env.get_string(&files_path).unwrap().into();

    let mut file_path = PathBuf::from(path_str);
    file_path.push("image");
    file_path.push(file_name);

    info!("file_path: {}", file_path.to_str().unwrap());

    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    buffer
}
