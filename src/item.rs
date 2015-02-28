use std::fmt::Write;
use syntax::ast;
use syntax::abi;
use syntax::parse::ParseSess;
use syntax::attr::AttrMetaMethods;

use ::Program;

pub fn translate(sess: &ParseSess, out: &mut String, prog: &mut Program, stages: &[&str], item: &ast::Item) {
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

            ::var::translate(sess, out, &item.attrs[..], item.ident,
                             &**ty, Some(&**expr), false);
        }

        ast::ItemConst(ref ty, ref expr) => {
            write!(out, "const ").unwrap();
            ::var::translate(sess, out, &item.attrs[..], item.ident,
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
                        if prog.vertex.is_none() {
                            prog.vertex = Some(String::new());
                        }
                        let vert: &mut String = prog.vertex.as_mut().unwrap();
                        ::shaders::translate(sess, vert, item.ident, "vertex", stages, &inputs[..], output, &**block);
                        return;
                    },
                    "fragment" => {
                        if prog.fragment.is_none() {
                            prog.fragment = Some(String::new());
                        }
                        let vert: &mut String = prog.fragment.as_mut().unwrap();
                        ::shaders::translate(sess, vert, item.ident, "fragment", stages, &inputs[..], output, &**block);
                        return;
                    },
                    "geometry" => {
                        if prog.geometry.is_none() {
                            prog.geometry = Some(String::new());
                        }
                        let sh: &mut String = prog.geometry.as_mut().unwrap();

                        if let Some(args) = attr.meta_item_list() {
                            for (i, arg) in args.iter().enumerate() {
                                match arg.node {
                                    ast::MetaWord(ref name) if i == 0 =>
                                        write!(sh, "layout({}) in;\n", name).unwrap(),
                                    ast::MetaNameValue(ref name, ref val) if i == 1 => {
                                        match val.node {
                                            ast::LitStr(ref val, _) =>
                                                write!(sh, "layout({}, max_vertices = {}) out;\n", name, val.parse::<u32>().unwrap()).unwrap(),
                                            _ => (),
                                        }
                                    },
                                    _ => (),
                                }
                            }
                        }

                        ::shaders::translate(sess, sh, item.ident, "geometry", stages, &inputs[..], output, &**block);
                        return;
                    },
                    _ => ()
                }
                //diag.span_err(attr.span, "no function attributes are supported");
            }

            ::fun::translate(sess, out, item.ident, &inputs[..], output, &**block);
        }

        ast::ItemMac(_) => {
            diag.span_bug(item.span, "macros should be gone by now");
        }

        ast::ItemStruct(ref def, ref generics) => {
            if generics.is_parameterized() {
                diag.span_err(item.span, "can't translate generic functions");
            }

            ::data::translate(sess, out, &item.attrs[..], item.ident, &def.fields[..]);
        }

        _ => {
            diag.span_err(item.span, "can't translate this sort of item");
        }
    }
}
