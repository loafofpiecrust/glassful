use syntax::{ast};
use std::borrow::ToOwned;

pub fn simple_path(p: &ast::Path) -> Option<String> {
    match &p.segments[..] {
        [ref single] => {
            let name = single.identifier.as_str().to_owned();
            if single.parameters.has_types() {
                match single.parameters.types()[0].node {
                    ast::TyPath(_, ref p) => Some(match &*simple_path(p).unwrap() {
                        "u32"|"usize" => "u",
                        "i32"|"isize" => "i",
                        "f64" => "d",
                        "bool" => "b",
                        _ => ""
                    }.to_owned() + &*name),
                    _ => None
                }
            }
            else { Some(name) }
        },
        _ => None
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
    let name = match &orig[..] {
        "f32" => "float",
        "f64" => "double",
        "i32"|"isize" => "int",
        "u32"|"usize" => "uint",
        name => name,
    };

    // convert Uppercase+Rust types to lowercase+glsl
    match name.to_lowercase() {
        ref n if n.len() >= 4 && (n.contains("pnt") || n.contains("rot")) => {
            if !(*n.as_bytes().last().unwrap() as char).is_numeric() {
                name.to_owned()
            }
            else {
                name.to_lowercase().replace("pnt", "vec").replace("rot", "vec")
            }
        },
        ref n if n.len() >= 4 && (n.contains("vec") || n.contains("mat")) => {
            if !(*n.as_bytes().last().unwrap() as char).is_numeric() {
                name.to_string()
            }
            else {
                n.clone()
            }
        },
        ref n if n.contains("sampler") => name[0..2].to_lowercase() + &name[2..],
        _ => name.to_owned()
    }
}
