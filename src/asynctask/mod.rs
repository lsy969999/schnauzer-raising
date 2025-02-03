use std::future::Future;

use bevy::prelude::*;
use futures::{
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    SinkExt,
};

pub struct AsyncTaskPlugin;

impl Plugin for AsyncTaskPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsyncTask::new());
        app.add_systems(Update, tick_runtime_update);

        app.insert_resource(Test2 { msg: String::new() });
        // app.add_systems(PostStartup, |asyn: ResMut<AsyncTask>| {
        //     asyn.spawn(hihi);
        // });
        // app.add_systems(Update, |test: ResMut<Test2>| {
        //     info!("test: {test:?}");
        // });
    }
}

#[derive(Resource, Debug)]
pub struct Test2 {
    pub msg: String,
}
#[derive(Resource)]
pub struct AsyncTask {
    #[cfg(not(target_family = "wasm"))]
    pub runtime: tokio::runtime::Runtime,
    update_run_tx: UnboundedSender<MainThreadCallback>,
    update_run_rx: UnboundedReceiver<MainThreadCallback>,
}
pub struct MainThreadContext<'a> {
    pub world: &'a mut World,
}
type MainThreadCallback = Box<dyn FnOnce(MainThreadContext) + Send + 'static>;
pub struct TaskCtx {
    pub update_run_tx: UnboundedSender<MainThreadCallback>,
}
impl TaskCtx {
    pub async fn run_on_main_thread<Runnable>(&mut self, runnable: Runnable)
    where
        Runnable: FnOnce(MainThreadContext) -> () + Send + 'static,
    {
        let (output_tx, output_rx) = futures::channel::oneshot::channel();
        let a = self.update_run_tx.send(Box::new(move |ctx| {
            if output_tx.send(runnable(ctx)).is_err() {
                panic!(
                    "Failed to sent output from operation run on main thread back to waiting task"
                );
            }
        }));
        if a.await.is_err() {
            panic!("Failed to send operation to be run on main thread");
        }

        output_rx.await.expect("run_on_main_thread output_rx err")
    }
}

impl AsyncTask {
    pub fn new() -> Self {
        #[cfg(not(target_family = "wasm"))]
        {
            let (update_run_tx, update_run_rx) = mpsc::unbounded::<MainThreadCallback>();
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime for background tasks");
            Self {
                runtime,
                update_run_rx,
                update_run_tx,
            }
        }
        #[cfg(target_family = "wasm")]
        {
            let (update_run_tx, update_run_rx) = mpsc::unbounded::<MainThreadCallback>();
            Self {
                update_run_rx,
                update_run_tx,
            }
        }
    }
    #[cfg(target_family = "wasm")]
    pub fn spawn<F, Spawnable>(&self, spawnable: Spawnable)
    where
        F: Future<Output = ()> + 'static,
        Spawnable: FnOnce(TaskCtx) -> F + Send + 'static,
    {
        let ctx = TaskCtx {
            update_run_tx: self.update_run_tx.clone(),
        };
        let future = spawnable(ctx);
        wasm_bindgen_futures::spawn_local(future);
    }
    #[cfg(not(target_family = "wasm"))]
    pub fn spawn<F, Spawnable>(&self, spawnable: Spawnable)
    where
        F: Future<Output = ()> + Send + 'static,
        Spawnable: FnOnce(TaskCtx) -> F + Send + 'static,
    {
        let ctx = TaskCtx {
            update_run_tx: self.update_run_tx.clone(),
        };
        let future = spawnable(ctx);
        self.runtime.spawn(future);
    }
}

fn tick_runtime_update(world: &mut World) {
    if let Some(mut a) = world.remove_resource::<AsyncTask>() {
        while let Ok(Some(f)) = a.update_run_rx.try_next() {
            let ctx = MainThreadContext { world };
            f(ctx);
        }
        world.insert_resource(a);
    }
}

async fn hihi(mut ctx: TaskCtx) {
    let client = reqwest::Client::new();
    let url = "https://jsonplaceholder.typicode.com/todos/1";

    match client.get(url).send().await {
        Ok(response) => {
            //
            info!("ok response: {response:?}");
            let text = response.text().await.unwrap();
            info!("text: {text:?}");

            ctx.run_on_main_thread(|ctx| {
                let test2 = ctx.world.get_resource_mut::<Test2>();
                if let Some(mut test2) = test2 {
                    test2.msg = text;
                }
            })
            .await;
        }
        Err(err) => {
            //
            info!("err: {err:?}");
        }
    }
}
