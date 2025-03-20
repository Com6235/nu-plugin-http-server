use std::{io::{stdout, Write}, vec};

use nu_plugin::{PluginCommand, EvaluatedCall, EngineInterface};
use nu_protocol::{ErrorLabel, Example, LabeledError, PipelineData, PipelineMetadata, Signature, SyntaxShape, Type, Value};

use crate::{router::{server::Server, Route}, HttpServerPlugin};

pub struct HttpServeCommand;

fn parse_pipeline_mime(pipeline: Option<PipelineMetadata>, default: &str) -> String {
    let mut str = pipeline
        .unwrap_or(PipelineMetadata::default())
        .content_type
        .unwrap_or(String::from(default));
    str.push_str("; charset=utf-8");
    str
}

impl PluginCommand for HttpServeCommand {
    type Plugin = HttpServerPlugin;

    fn name(&self) -> &str {
        "http serve"
    }

    fn description(&self) -> &str {
        "Serve piped data on a web server"
    }

    fn examples(&self) -> Vec<nu_protocol::Example> {
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

    fn signature(&self) -> nu_protocol::Signature {
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

        let (data, given_mime): (Vec<u8>, String) = match input {
            PipelineData::Empty => (vec![], String::from("text/plain; charset=utf-8")),
            PipelineData::Value(val, meta) => {
                match val {
                    Value::String { val, internal_span: _ } => (val.as_bytes().to_vec(), parse_pipeline_mime(meta, "text/plain")),
                    Value::Nothing { internal_span: _ } => (vec![], String::from("text/plain")),
                    Value::Bool { val, internal_span: _ } => ((if val { "true" } else { "false" }).as_bytes().to_vec(), parse_pipeline_mime(meta, "text/plain")),
                    Value::Binary { val, internal_span: _ } => (val, parse_pipeline_mime(meta, "application/octet-stream")),
                    _ => {
                        return Err(LabeledError { msg: String::from("Not Implemented! (Value::*)"), labels: Box::new(vec![ErrorLabel { span: val.span(), text: String::from("Unimplemented data type.") }]), code: None, url: None, help: None, inner: Box::new(vec![]) });
                    }
                } 
            }           
            PipelineData::ListStream(_val, _) => {
                return Err(LabeledError { msg: String::from("Not Implemented! (PipelineData::ListStream)"), labels: Box::new(vec![]), code: None, url: None, help: None, inner: Box::new(vec![]) });
            }
            PipelineData::ByteStream(val, meta) => {
                let a = val.into_value().unwrap();
                match a {
                    Value::Binary { val, internal_span: _ } => (val, parse_pipeline_mime(meta, "application/octet-stream")),
                    Value::String { val, internal_span: _ } => (val.as_bytes().to_vec(), parse_pipeline_mime(meta, "text/plain")),
                    _ => { return Err(LabeledError { msg: String::from("How did you get here?"), labels: Box::new(vec![ErrorLabel { span: a.span(), text: String::from("Literally how?") }]), code: None, url: None, help: None, inner: Box::new(vec![]) }); }
                }
            }
        };

        let mime = call.get_flag::<String>("mime").unwrap_or(None).unwrap_or(given_mime);

        plugin.async_runtime.block_on(async {
            let (task, local_addr) = Server::new(bind_addr, vec![Route::new(String::from("/"), data, mime)]).start().await;

            let (tx, rx) = tokio::sync::oneshot::channel::<bool>();
            let signal = engine.register_signal_handler(Box::new(move |_| { let _ = tx.clone().send(true); })).unwrap();

            if !engine.is_using_stdio() {
                let _ = stdout().write_all(String::from(format!("Binded to: http://{}:{}/", local_addr.ip().to_string(), local_addr.port())).as_bytes());
                let _ = stdout().flush();
            }

            let _ = rx.await;
            task.abort();
            let _ = guard.leave();
            drop(signal);

            Ok(PipelineData::Empty)
        })
    }
}

  