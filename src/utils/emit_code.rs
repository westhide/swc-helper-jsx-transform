use swc_core::{
    common::{sync::Lrc, SourceMap},
    ecma::codegen::{text_writer::JsWriter, Config, Emitter, Node},
};

pub fn emit_code<N: Node>(node: &N) -> String {
    let cm = Lrc::<SourceMap>::default();
    let mut code: Vec<u8> = Vec::new();

    let mut emitter = Emitter {
        cfg: Config::default(),
        cm: cm.clone(),
        comments: None,
        wr: JsWriter::new(cm, "\n", &mut code, None),
    };

    node.emit_with(&mut emitter).unwrap();

    String::from_utf8(code).unwrap()
}
