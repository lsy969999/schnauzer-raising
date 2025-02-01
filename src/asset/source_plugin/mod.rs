use bevy::asset::io::{AssetReader, AssetSource};
use bevy::prelude::*;

pub struct FilesAssetPlugin;

impl Plugin for FilesAssetPlugin {
    fn build(&self, app: &mut App) {
        // app.register_asset_source(
        //     "files",
        //     AssetSource::build().with_reader(|| Box::new(FilesAssetReader)),
        // );
    }
}

// pub struct FilesAssetReader;

// impl AssetReader for FilesAssetReader {
//     fn read<'a>(
//         &'a self,
//         path: &'a std::path::Path,
//     ) -> impl bevy::asset::io::AssetReaderFuture<Value: bevy::asset::io::Reader + 'a> {
//         todo!()
//     }

//     fn read_meta<'a>(
//         &'a self,
//         path: &'a std::path::Path,
//     ) -> impl bevy::asset::io::AssetReaderFuture<Value: bevy::asset::io::Reader + 'a> {
//         todo!()
//     }

//     fn read_directory<'a>(
//         &'a self,
//         path: &'a std::path::Path,
//     ) -> impl bevy::utils::ConditionalSendFuture<
//         Output = Result<Box<bevy::asset::io::PathStream>, bevy::asset::io::AssetReaderError>,
//     > {
//         todo!()
//     }

//     fn is_directory<'a>(
//         &'a self,
//         path: &'a std::path::Path,
//     ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<bool, bevy::asset::io::AssetReaderError>>
//     {
//         todo!()
//     }
// }
