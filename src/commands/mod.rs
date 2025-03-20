use nu_protocol::{ByteStream, ErrorLabel, LabeledError, ListStream, PipelineData, PipelineMetadata, Value};

pub mod serve;

fn parse_pipeline_data(input: PipelineData) -> Result<(Vec<u8>, String), LabeledError> {
    match input {
        PipelineData::Empty => Ok((vec![], String::from("text/plain; charset=utf-8"))),
        PipelineData::Value(val, meta) => parse_value(val, meta),
        PipelineData::ListStream(val, meta) => parse_list_stream(val, meta),
        PipelineData::ByteStream(val, meta) => parse_byte_stream(val, meta)
    }
}

fn parse_pipeline_mime(pipeline: Option<PipelineMetadata>, default: &str) -> String {
    let mut str = pipeline
        .unwrap_or(PipelineMetadata::default())
        .content_type
        .unwrap_or(String::from(default));

    str.push_str("; charset=utf-8");

    str
}

fn parse_value(val: Value, meta: Option<PipelineMetadata>) -> Result<(Vec<u8>, std::string::String), LabeledError> {
    let a = match val {
        Value::String { val, internal_span: _ } => (val.as_bytes().to_vec(), parse_pipeline_mime(meta, "text/plain")),
        Value::Nothing { internal_span: _ } => (vec![], String::from("text/plain")),
        Value::Bool { val, internal_span: _ } => ((if val { "true" } else { "false" }).as_bytes().to_vec(), parse_pipeline_mime(meta, "text/plain")),
        Value::Binary { val, internal_span: _ } => (val, parse_pipeline_mime(meta, "application/octet-stream")),
        _ => {
            return Err(LabeledError { msg: String::from("Not Implemented! (Value::*)"), labels: Box::new(vec![ErrorLabel { span: val.span(), text: String::from("Unimplemented data type.") }]), code: None, url: None, help: None, inner: Box::new(vec![]) });
        }
    };
    Ok(a)
}

fn parse_byte_stream(val: ByteStream, meta: Option<PipelineMetadata>) -> Result<(Vec<u8>, String), LabeledError> {
    let a = val.into_value().unwrap();
    match a {
        Value::Binary { val, internal_span: _ } => Ok((val, parse_pipeline_mime(meta, "application/octet-stream"))),
        Value::String { val, internal_span: _ } => Ok((val.as_bytes().to_vec(), parse_pipeline_mime(meta, "text/plain"))),
        _ => Err(LabeledError { msg: String::from("How did you get here?"), labels: Box::new(vec![ErrorLabel { span: a.span(), text: String::from("Literally how?") }]), code: None, url: None, help: None, inner: Box::new(vec![]) })
    }
}

fn parse_list_stream(_val: ListStream, _meta: Option<PipelineMetadata>) -> Result<(Vec<u8>, String), LabeledError> {
    return Err(LabeledError { msg: String::from("Not Implemented! (PipelineData::ListStream)"), labels: Box::new(vec![]), code: None, url: None, help: None, inner: Box::new(vec![]) });
}
