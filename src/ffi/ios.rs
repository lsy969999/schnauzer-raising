use std::{
    ffi::CStr,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use bevy::prelude::*;
use objc2::{class, msg_send, rc::Retained, runtime::NSObject};

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
                            ..default()
                        }),
                        ..default()
                    }),
            )
            .add_plugins(crate::app::AppPlugin)
            .insert_resource(bevy::winit::WinitSettings::mobile())
            .add_systems(PostStartup, || {
                //
                let path = get_application_support_path();
                info!("path: {:?}", path);
                copy_assets_to_application_support();
            })
            .run();
    }
}

fn get_application_support_path() -> String {
    // let file_manager: Retained<NSObject> =
    //     unsafe { msg_send![class!(NSFileManager), defaultManager] };
    // let urls: Retained<NSArray<NSURL>> = unsafe {
    //     msg_send![file_manager, URLsForDirectory:5, inDomains:1] // 5 = Application Support
    // };
    // use objc2_file_provider::NSFileProviderExtension;

    // let a = unsafe { objc2_file_provider::NSFileProviderManager::defaultManager() };
    // a.getUserVisibleURLForItemIdentifier_completionHandler(item_identifier, completion_handler);

    // let a = unsafe { objc2_file_provider::NSFileProviderManager::defaultManager() };

    unsafe {
        let file_manager: Retained<NSObject> = msg_send![class!(NSFileManager), defaultManager];
        info!("filemanager: {file_manager:?}");
        let directory = 14u64; // NSSearchPathDirectory 값은 u64 타입이어야 함
        let domain_mask = 1u64; // NSUserDomainMask 값도 u64 타입이어야 함
        let urls: Retained<NSObject> =
            msg_send![&file_manager, URLsForDirectory: directory, inDomains: domain_mask];
        info!("urls {:?}", urls);
        let url: Retained<NSObject> = msg_send![&urls, firstObject];
        info!("url {:?}", url);
        let path: Retained<NSObject> = msg_send![&url, path];
        info!("path {:?}", path);

        let utf8_ptr: *const std::os::raw::c_char = msg_send![&path, UTF8String];
        let rust_string = CStr::from_ptr(utf8_ptr).to_string_lossy().into_owned();
        rust_string
    }
}

fn copy_assets_to_application_support() {
    //
    unsafe {
        let main_bundle: Retained<NSObject> = msg_send![class!(NSBundle), mainBundle];
        let assets_path: Retained<NSObject> = msg_send![&main_bundle, resourcePath];

        let assets: Retained<NSObject> =
            msg_send![class!(NSString), stringWithUTF8String: c"assets".as_ptr()];
        info!("------");
        info!("assets {:?}", assets);
        // let utf8_ptr: *const std::os::raw::c_char = msg_send![&assets, UTF8String];
        // let rust_string = CStr::from_ptr(utf8_ptr).to_string_lossy().into_owned();
        // info!("assets: {}", rust_string);
        // let assetsPath2: Retained<NSObject> =
        //     msg_send![&*assetsPath, stringByAppendingPathComponent: &*assets];
        // info!("------");
        // info!("assetsPath {:?}", assetsPath);
        // info!("------");
        let utf8_ptr: *const std::os::raw::c_char = msg_send![&assets_path, UTF8String];
        let rust_string = CStr::from_ptr(utf8_ptr).to_string_lossy().into_owned();

        let mut file_path = PathBuf::from(rust_string);
        file_path.push("assets");
        file_path.push("image");
        info!("file_path: {file_path:?}");

        if std::fs::metadata(&file_path).is_ok() {
            info!("directory ok");
        } else {
            info!("directory no");
        }

        if !file_path.exists() {
            info!("file_path not exists");
            return;
        }

        // let entries = fs::read_dir(file_path).ok().unwrap();
        // let mut files = Vec::new();
        // for entry in entries {
        //     if let Ok(entry) = entry {
        //         let file_name = entry
        //             .file_name()
        //             .into_string()
        //             .unwrap_or_else(|_| "Invalid UTF-8".to_string());
        //         files.push(file_name);
        //     }
        // }

        // info!("files: {files:?}");
        file_path.push("bevy_icon.png");

        // info!("file_path: {file_path:?}");

        let mut file = File::open(file_path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        info!("read from imgbyte: {}", buffer.len());

        let file_manager: Retained<NSObject> = msg_send![class!(NSFileManager), defaultManager];

        let urls: Retained<NSObject> =
            msg_send![&file_manager, URLsForDirectory: 14u64, inDomains: 1u64];

        let url: Retained<NSObject> = msg_send![&urls, firstObject];

        let path: Retained<NSObject> = msg_send![&url, path];

        let utf8_ptr: *const std::os::raw::c_char = msg_send![&path, UTF8String];
        let rust_string = CStr::from_ptr(utf8_ptr).to_string_lossy().into_owned();

        let mut new_path = PathBuf::from(rust_string);
        new_path.push("image");

        if !&new_path.exists() {
            fs::create_dir_all(&new_path).unwrap();
        }

        let new_file = new_path.join("bevy_icon.png");
        info!("new file: {new_file:?}");
        let mut file = File::create(&new_file).unwrap();
        file.write_all(&buffer).unwrap();

        let mut file = File::open(new_file).unwrap();
        let mut buffer2 = Vec::new();
        file.read_to_end(&mut buffer2).unwrap();
        info!("wow buffer2: {:?}", buffer2.len());
    }
}
