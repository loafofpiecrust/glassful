use std::fmt::Write;
use syntax::ast;
use syntax::parse::ParseSess;
use syntax::attr::AttrMetaMethods;

#[allow(unused_assignments)]
pub fn translate(sess: &ParseSess,
                 out: &mut String,
                 attrs: &[ast::Attribute],
                 ident: ast::Ident,
                 ty: &ast::Ty,
                 mut init: Option<&ast::Expr>,
                 local: bool) {
    let diag = &sess.span_diagnostic;

    let mut attributed = false;
    for attr in attrs.iter() {
        attributed = true;
        let name = &*attr.name();
        let mut in_idx = 0;
        let mut out_idx = 0;
        match name {
            // many others: https://www.opengl.org/wiki/Type_Qualifier_%28GLSL%29
            "uniform" => {
                write!(out, "{} ", name).unwrap();
            }
            "attribute" => {
                write!(out, "{} ", name).unwrap();
                in_idx += 1;
            }
            "varying" => {
                write!(out, "{} ", name).unwrap();
                out_idx += 1;
            }
            "input" => {
                write!(out, "layout(location={}) in ", in_idx).unwrap();
                in_idx += 1;
            }
            "output" => {
                write!(out, "layout(location={}) out ", out_idx).unwrap();
                out_idx += 1;
            }
            _ => diag.span_err(attr.span, "unknown variable attribute"),
        }
    }

    if !attributed && !local {
        write!(out, "uniform ").unwrap();
    }

    // The special ident 'UNINIT' means no initializer.
    // Rust's syntax does not allow this otherwise on statics.
    if let Some(i) = init {
        if let ast::ExprPath(_, ref p) = i.node {
            if let Some(s) = ::util::simple_path(p) {
                if &s[..] == "UNINIT" {
                    init = None;
                }
            }
            else {
                // NOTE: Is this invalid?
                init = None;
            }
        }
    }

    if let ast::TyVec(ref t) = ty.node {
        ::ty::translate(sess, out, &*t);
        write!(out, " {}[]", ident.as_str()).unwrap();
    }
    else {
        ::ty::translate(sess, out, ty);
        write!(out, " {}", ident.as_str()).unwrap();
    }
    if let Some(init) = init {
        write!(out, " = ").unwrap();
        ::expr::translate(sess, out, init);
    }

    if !local { write!(out, ";\n").unwrap(); }
}
