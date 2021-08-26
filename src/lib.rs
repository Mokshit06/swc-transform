#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
/// Explicit extern crate to use allocator.
extern crate swc_node_base;

use napi::{CallContext, Env, JsObject, JsUndefined, Result};
use std::sync::Arc;
use swc::{Compiler, TransformOutput};
use swc_common::{
    //     self,
    errors::{ColorConfig, Handler},
    sync::Lazy,
    FilePathMapping,
    SourceMap,
};

mod ast_transform;
mod transform;
mod util;

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Always,
        true,
        false,
        Some(cm.clone()),
    ));

    Arc::new(Compiler::new(cm.clone(), handler))
});

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("transform", transform::transform)?;
    exports.create_named_method("transformSync", transform::transform_sync)?;

    Ok(())
}

fn get_compiler(_ctx: &CallContext) -> Arc<Compiler> {
    COMPILER.clone()
}

// #[js_function]
// fn construct_compiler(ctx: CallContext) -> Result<JsUndefined> {
//     // TODO: Assign swc::Compiler
//     ctx.env.get_undefined()
// }

pub fn complete_output(env: &Env, output: TransformOutput) -> Result<JsObject> {
    env.to_js_value(&output)?.coerce_to_object()
}

// pub type ArcCompiler = Arc<Compiler>;
