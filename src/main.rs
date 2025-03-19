use commands::serve::HttpServeCommand;
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};
use router::Route;
use tokio::runtime;

mod commands;
mod router;

pub struct HttpServerPlugin {
    async_runtime: runtime::Runtime,
    routes: Vec<Route>
}

impl Plugin for HttpServerPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin=Self>>> {
        vec![Box::new(HttpServeCommand)]
    }
}

fn main() {
    let runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let plugin = HttpServerPlugin {
        async_runtime: runtime,
        routes: vec![]
    };

    serve_plugin(&plugin, MsgPackSerializer);
}