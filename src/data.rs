use std::fmt::Write;
use syntax::ast;
use syntax::parse::ParseSess;
use syntax::attr::AttrMetaMethods;


pub fn translate(sess: &ParseSess,
				 out: &mut String,
				 _: &[ast::Attribute],
				 ident: ast::Ident,
				 fields: &[ast::StructField],
				) {
	let diag = &sess.span_diagnostic;

	writeln!(out, "struct {} {{", ident.name.as_str()).unwrap();
	for field in fields {
		match field.node.kind {
			ast::NamedField(ident, _) => {
				write!(out, "\t").unwrap();
				::ty::translate(sess, out, &*field.node.ty);
				writeln!(out, " {};", ident.name).unwrap()
			}
			_ => diag.span_err(field.span, "Struct fields must be named.")
		}
	}
	writeln!(out, "}};").unwrap();
}