use syntax::ast;
use std::borrow::ToOwned;
use std::ascii::AsciiExt;

pub fn simple_path(p: &ast::Path) -> Option<String> {
    match &p.segments[..] {
        [ref single] if single.parameters.is_empty()
            => Some(single.identifier.as_str().to_owned()),
        _ => None,
    }
}

pub fn pat_to_var(p: &ast::Pat) -> Option<String> {
    match p.node {
        ast::PatIdent(ast::BindByValue(_), id, None)
            => Some(id.node.as_str().to_owned()),
        _ => None,
    }
}

pub fn pat_to_ident(p: &ast::Pat) -> Option<ast::Ident> {
    match p.node {
        ast::PatIdent(_, id, None) => Some(id.node),
        _ => None
    }
}

pub fn convert_type(orig: &String) -> String {
    println!("input type: {}", orig);
    let name = match &orig[..] {
        "f32" => "float",
        "f64" => "double",
        "i32" => "int",
        "u32" => "uint",
        name => name,
    };
    // convert Uppercase+Rust types to lowercase+glsl
    match name.to_ascii_lowercase() {
        ref n if n.len() == 4 && (n.starts_with("pnt") || n.starts_with("rot")) => {
            if !(*n.as_bytes().last().unwrap() as char).is_numeric() {
                name.to_string()
            }
            else {
                let mut start = "vec".to_string();
                start.push(*n.as_bytes().last().unwrap() as char);
                start
            }
        },
        ref n if n.len() == 4 && (n.starts_with("vec") || n.starts_with("mat")) => {
            if !(*n.as_bytes().last().unwrap() as char).is_numeric() {
                name.to_string()
            }
            else {
                n.clone()
            }
        },
        _ => name.to_string(),
    }
}