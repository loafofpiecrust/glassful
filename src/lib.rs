#![feature(rustc_private)]
#![deny(warnings)]
#![allow(dead_code)]

extern crate syntax;

use std::borrow::ToOwned;
use std::thread;
use syntax::parse;
use syntax::ext::expand;
use syntax::attr::AttrMetaMethods;
use syntax::ast;

mod item;
mod var;
mod util;
mod ty;
mod fun;
mod expr;
mod block;
mod shaders;
mod data;

const NAME: &'static str = "<glassful shader>";

pub struct Program {
    pub vertex: Option<String>,
    pub fragment: Option<String>,
    pub geometry: Option<String>,
}

impl Program {
    fn new() -> Self {
        Program {
            vertex: None,
            fragment: None,
            geometry: None,
        }
    }

    fn merge_template(&mut self, template: &String) {
        self.vertex = self.vertex.as_ref().map(|sh| template.clone() + &sh[..]);
        self.fragment = self.fragment.as_ref().map(|sh| template.clone() + &sh[..]);
        self.geometry = self.geometry.as_ref().map(|sh| template.clone() + &sh[..]);
    }
}

/// Translate a glassful program to GLSL, or panic.
pub fn translate(source: String) -> Program {
    // parse
    let sess = parse::new_parse_sess();
    let diag = &sess.span_diagnostic;
    let krate = parse::parse_crate_from_source_str(
        NAME.to_owned(), source, vec![], &sess);

    diag.handler.abort_if_errors();

    // expand macros
    let ecfg = expand::ExpansionConfig::default(NAME.to_owned());
    let krate = expand::expand_crate(&sess, ecfg, vec![], vec![], krate);

    // process attributes
    let mut glsl_version = None;
    for attr in krate.attrs.iter() {
        if attr.check_name("version") {
            if let Some(val) = attr.value_str() {
                if glsl_version.is_some() {
                    diag.span_err(attr.span, "version given twice");
                }
                glsl_version = Some((*val).to_string());
            } else {
                diag.span_err(attr.span, "version not given");
            }
        } else {
            diag.span_err(attr.span, "unknown attribute");
        }
    }

    diag.handler.abort_if_errors();

    // translate!

    let mut template = match glsl_version {
        Some(v) => format!("#version {}\n\n", v),
        None => "".to_owned(),
    };;
    let mut shaders = Program::new();

    // gather list of stages
    let mut stages = vec![];
    for item in &krate.module.items {
        let item = &**item;
        if let ast::ItemFn(..) = item.node {
            for attr in &item.attrs {
                match &*attr.name() {
                    "vertex" => stages.push("vertex"),
                    "fragment" => stages.push("fragment"),
                    "geometry" => stages.push("geometry"),
                    _ => ()
                }
            }
        }
    }

    for item in &krate.module.items {
        item::translate(&sess, &mut template, &mut shaders, &stages[..], &**item);
    }

    shaders.merge_template(&template);

    diag.handler.abort_if_errors();
    shaders
}

/// Translate a glassful program to GLSL, or return `None'.
///
/// Because the `libsyntax` parser uses `panic!` internally,
/// this spawns a new thread for the duration of the call.
pub fn try_translate(source: String) -> Option<Program> {
    let result = translate(source);
    if thread::panicking() {
        None
    }
    else { Some(result) }
}