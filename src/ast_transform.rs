use swc_common::pass::{Repeat, Repeated};
use swc_common::{FileName, DUMMY_SP};
use swc_ecmascript::ast::*;
use swc_ecmascript::visit::Fold;

pub fn ast_transform(filename: FileName) -> impl Fold {
    AstTransform { filename }
}

struct AstTransform {
    filename: FileName,
}

impl Fold for AstTransform {
    fn fold_export_named_specifier(&mut self, s: ExportNamedSpecifier) -> ExportNamedSpecifier {
        println!("{:?}", s);
        s
    }
}
