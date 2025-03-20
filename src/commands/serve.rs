use std::{io::{stdout, Write}, vec};

use nu_plugin::{PluginCommand, EvaluatedCall, EngineInterface};
use nu_protocol::{Example, LabeledError, PipelineData, Signature, SyntaxShape, Type};

use crate::{router::{server::Server, Route}, HttpServerPlugin};
use super::parse_pipeline_data;

pub struct HttpServeCommand;

impl PluginCommand for HttpServeCommand {
    type Plugin = HttpServerPlugin;

    fn name(&self) -> &str {
        "http serve"
    }

    fn description(&self) -> &str {
        "Serve piped data on a web server"
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "List all files as JSON",
                example: "ls | to json | http serve --mime \'application/json\'",
                result: None
            },
            Example {
                description: "Serve a HTML document on port 25565",
                example: "open index.html --raw | http serve -m \'text/html\' -a \'127.0.0.1:25565\'",
                result: None
            },
        ]
    }

    fn extra_description(&self) -> &str {
        "Binds a HTTP server to a random (or given) address and port."
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .named("address", SyntaxShape::String, "Address to bind onto", Some('a'))
            .named("mime", SyntaxShape::String, "Mime-type to output", Some('m'))
            .input_output_types(vec![
                (Type::String, Type::Nothing),
                (Type::Nothing, Type::Nothing),
                (Type::Binary, Type::Nothing),
                (Type::Bool, Type::Nothing),
            ])
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let guard = engine.enter_foreground().unwrap();
        
        let bind_addr = call.get_flag::<String>("address").unwrap_or(None).unwrap_or(String::from("127.0.0.1:0"));

        let (data, given_mime) = match parse_pipeline_data(input) {
            Ok(value) => value,
            Err(value) => return Err(value),
        };

        let mime = call.get_flag::<String>("mime").unwrap_or(None).unwrap_or(given_mime);

        plugin.async_runtime.block_on(async {
            let server = Server::new(bind_addr, vec![Route::new(String::from("/"), data, mime)]);
            let task = server.start().await;

            let (tx, rx) = tokio::sync::oneshot::channel::<bool>();
            let signal = engine.register_signal_handler(Box::new(move |_| { let _ = tx.clone().send(true); })).unwrap();

            if !engine.is_using_stdio() {
                let addr = server.binded_address().unwrap();
                let _ = stdout().write_all(String::from(format!("Binded to: http://{}:{}/", addr.ip().to_string(), addr.port())).as_bytes());
                let _ = stdout().flush();
            }

            let _ = rx.await;
            task.abort();
            let _ = guard.leave();
            drop(signal);
        });

        Ok(PipelineData::Empty)
    }
}
