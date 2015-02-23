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
    let name = match &orig[..] {
        "f32" => "float",
        "f64" => "double",
        "i32" => "int",
        "u32" => "uint",
        name => name,
    };
    // convert Uppercase types to lowercase
    match name {
        n if n.starts_with("Vec")
           | n.starts_with("Mat")
           => n.to_ascii_lowercase(),
        n if n.starts_with("Pnt") || n.starts_with("Rot") => {
            let mut start = "vec".to_string();
            let mut n = n.to_string();
            start.push(n.pop().unwrap());
            start
        }
        n => n.to_string()
    }
}