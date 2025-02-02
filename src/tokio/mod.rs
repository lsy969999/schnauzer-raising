use bevy::prelude::*;

pub struct TokioPlugin;

impl Plugin for TokioPlugin {
    fn build(&self, app: &mut App) {
        //
    }
}

// fn test() {
//     let thread_pool = AsyncComputeTaskPool::get();

//     // let task = thread_pool.spawn(async move {
//     //     let client = reqwest::Client::new();
//     //     let url = "https://jsonplaceholder.typicode.com/todos/1";

//     //     match client.get(url).send().await {
//     //         Ok(response) => {
//     //             //
//     //             info!("ok response: {response:?}");
//     //         }
//     //         Err(err) => {
//     //             //
//     //             info!("err: {err:?}");
//     //         }
//     //     }
//     //     //
//     // });
//     let rt = Runtime::new().unwrap();
//     rt.block_on(async {
//         tokio::spawn(async {
//             let client = reqwest::Client::new();
//             let url = "https://jsonplaceholder.typicode.com/todos/1";

//             match client.get(url).send().await {
//                 Ok(response) => {
//                     //
//                     info!("ok response: {response:?}");
//                 }
//                 Err(err) => {
//                     //
//                     info!("err: {err:?}");
//                 }
//             }
//         })
//         .await
//         .unwrap();
//     });

//     // tokio::task::spawn_blocking(task);
// }
