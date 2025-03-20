use commands::serve::HttpServeCommand;
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};
use tokio::runtime;

mod commands;
mod router;

pub struct HttpServerPlugin {
    async_runtime: runtime::Runtime,
}

impl Plugin for HttpServerPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin=Self>>> {
        vec![Box::new(HttpServeCommand)]
    }
}

impl HttpServerPlugin {
    pub fn new() -> HttpServerPlugin {
        let runtime = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        HttpServerPlugin { async_runtime: runtime }
    }
}

fn main() {
    serve_plugin(&HttpServerPlugin::new(), MsgPackSerializer);
}