#![crate_type="dylib"]
#![feature(plugin_registrar)]
#![feature(rustc_private, core)]
#![deny(warnings)]
#![allow(unused_features)]

extern crate syntax;
extern crate rustc;
extern crate glassful;

use syntax::ast;
use syntax::parse::token;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, MacExpr, DummyResult};
use syntax::ext::build::AstBuilder;
use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("glassful", expand);
}

fn expand(cx: &mut ExtCtxt, outer_span: Span, toks: &[ast::TokenTree])
    -> Box<MacResult + 'static>
{
    let inner_span = match toks {
        [] => {
            cx.span_err(outer_span, "empty invocation");
            return DummyResult::expr(outer_span);
        }
        [ref first, ..] => {
            let first = first.get_span();
            let last = toks.iter().rev().next().unwrap().get_span();
            if first.expn_id != last.expn_id {
                cx.span_err(first, "invocation is split between expansion contexts??");
                cx.span_note(last, "last token is here");
                return DummyResult::expr(outer_span);
            }

            Span {
                lo: first.lo,
                hi: last.hi,
                expn_id: first.expn_id,
            }
        }
    };

    let src = match cx.codemap().span_to_snippet(inner_span) {
        Err(e) => {
            cx.span_err(inner_span, &format!("can't extract source snippet: {:?}", e)[..]);
            return DummyResult::expr(inner_span);
        }
        Ok(src) => src,
    };

    match glassful::try_translate(src) {
        None => {
            cx.span_err(outer_span, "translation failed");
            DummyResult::expr(outer_span)
        }
        Some(res) => {
            /*let interned = token::intern_and_get_ident(res);
            MacExpr::new(cx.expr_str(inner_span, interned))*/
            let mut sh_vec = vec![];
            for sh in &[res.vertex, res.fragment, res.geometry] {
                if let Some(sh) = sh.clone() {
                    let sh = token::intern_and_get_ident(&(sh)[..]);
                    sh_vec.push(cx.expr_str(inner_span, sh));
                }
            }

            if sh_vec.len() == 1 {
                MacExpr::new(sh_vec[0].clone())
            }
            else {
                MacExpr::new(cx.expr_vec(inner_span, sh_vec))
            }
        }
    }
}
