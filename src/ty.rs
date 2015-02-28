use std::fmt::Write;
use std::slice::SliceExt;
use syntax::ast;
use syntax::parse::ParseSess;
use syntax::attr::AttrMetaMethods;

pub fn translate(sess: &ParseSess,
                 out: &mut String,
                 ty: &ast::Ty) {
    let diag = &sess.span_diagnostic;

    match ty.node {
        ast::TyTup(ref t) if t.len() == 0 => {
            write!(out, "void").unwrap();
        }

        ast::TyPath(_, ref p) => match ::util::simple_path(p) {
            None => {
                diag.span_err(ty.span, "can't translate qualified / parametrized name");
            }
            Some(name) => {
                write!(out, "{}", ::util::convert_type(&name)).unwrap()
            }
        },

        ast::TyVec(ref t) => {
            translate(sess, out, t)
        }

        _ => {
            diag.span_err(ty.span, "can't translate this sort of type");
        }
    }
}
