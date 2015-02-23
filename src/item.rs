use std::fmt::Write;
use syntax::ast;
use syntax::abi;
use syntax::parse::ParseSess;
use syntax::attr::AttrMetaMethods;

use shaders::Shaders;

pub fn translate(sess: &ParseSess, out: &mut Shaders, item: &ast::Item) {
    let diag = &sess.span_diagnostic;

    match item.vis {
        ast::Visibility::Inherited => (),
        _ => diag.span_err(item.span, "`pub` visibility has no meaning"),
    }

    match item.node {
        ast::ItemStatic(ref ty, muta, ref expr) => {
            match muta {
                ast::MutImmutable => (),
                _ => diag.span_err(item.span, "variables are implicitly mutable"),
            }

            ::var::translate(sess, &mut out.template, &item.attrs[..], item.ident,
                             &**ty, Some(&**expr), false);
        }

        ast::ItemConst(ref ty, ref expr) => {
            write!(&mut out.template, "const ").unwrap();
            ::var::translate(sess, &mut out.template, &item.attrs[..], item.ident,
                             &**ty, Some(&**expr), false);
        }

        ast::ItemFn(ref decl, unsafety, abi, ref generics, ref block) => {
            let ast::FnDecl { ref inputs, ref output, variadic }
                = **decl;

            if variadic {
                diag.span_err(item.span, "can't translate variadic functions");
            }

            match unsafety {
                ast::Unsafety::Normal => (),
                _ => diag.span_err(item.span, "can't translate unsafe functions"),
            }

            match abi {
                abi::Abi::Rust => (),
                _ => diag.span_err(item.span, "can't translate non-default ABI"),
            }

            if generics.is_parameterized() {
                diag.span_err(item.span, "can't translate generic functions");
            }

            let output = match *output {
                ast::NoReturn(..) => {
                    diag.span_err(item.span, "function doesn't return");
                    return;
                }
                ast::DefaultReturn(..) => None,
                ast::Return(ref t) => Some(&**t),
            };

            for attr in item.attrs.iter() {
                let name = &*attr.name();
                match name {
                    "vertex" => {
                        if out.vertex.is_none() {
                            out.vertex = Some(String::new());
                        }
                        let vert: &mut String = out.vertex.as_mut().unwrap();
                        ::shaders::translate(sess, vert, item.ident, "vertex", &inputs[..], output, &**block);
                        return;
                    },
                    "fragment" => {
                        if out.fragment.is_none() {
                            out.fragment = Some(String::new());
                        }
                        let vert: &mut String = out.fragment.as_mut().unwrap();
                        ::shaders::translate(sess, vert, item.ident, "fragment", &inputs[..], output, &**block);
                        return;
                    },
                    "geometry" => {
                        if out.geometry.is_none() {
                            out.geometry = Some(String::new());
                        }
                        let vert: &mut String = out.geometry.as_mut().unwrap();
                        ::shaders::translate(sess, vert, item.ident, "geometry", &inputs[..], output, &**block);
                        return;
                    },
                    _ => ()
                }
                //diag.span_err(attr.span, "no function attributes are supported");
            }

            ::fun::translate(sess, &mut out.template, item.ident, &inputs[..], output, &**block);
        }

        ast::ItemMac(_) => {
            diag.span_bug(item.span, "macros should be gone by now");
        }

        ast::ItemStruct(ref def, ref generics) => {
            if generics.is_parameterized() {
                diag.span_err(item.span, "can't translate generic functions");
            }

            ::data::translate(sess, &mut out.template, &item.attrs[..], item.ident, &def.fields[..]);
        }

        _ => {
            diag.span_err(item.span, "can't translate this sort of item");
        }
    }
}
