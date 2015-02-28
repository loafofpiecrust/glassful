use std::fmt::Write;
use syntax::ast;
use syntax::parse::ParseSess;

pub struct Program {
    pub vertex: Option<String>,
    pub fragment: Option<String>,
    pub geometry: Option<String>
}

pub fn translate(sess: &ParseSess,
				 out: &mut String,
				 name: ast::Ident,
				 stage: &'static str,
                 _: &[&str],
				 inputs: &[ast::Arg],
				 output: Option<&ast::Ty>,
				 block: &ast::Block,
				) {
    //let diag = &sess.span_diagnostic;

    /*let last_stage = match stage {
        "fragment" if stages.contains(&"geometry") => Some("geometry"),
        "fragment" if stages.contains(&"vertex") => Some("vertex"),
        "geometry" if stages.contains(&"vertex") => Some("vertex"),
        _ => None,
    };*/

    let mut idx = 0;
    let mut mut_idx = 0;
    for (_, arg) in inputs.iter().enumerate() {
        /*if i != 0 {
            write!(out, ", ").unwrap();
        }*/

        /*if let Some(last_stage) = last_stage {
            if i == 0 {
                write!(out, "in {};\n", last_stage).unwrap();
                idx += 1;
                continue;
            }
        }*/

        match arg.pat.node.clone() {
        	ast::PatIdent(ast::BindByValue(ast::MutMutable), _, _) => {
                if stage == "fragment" {
                    write!(out, "layout(location={}) ", mut_idx).unwrap();
                }
        		write!(out, "out ").unwrap();
                mut_idx += 1;
        	}
        	_ => {
                if stage == "vertex" {
                    write!(out, "layout(location={}) ", idx).unwrap();
                }
		        write!(out, "in ").unwrap();
                idx += 1;
        	}
        }
        //::ty::translate(sess, out, &*arg.ty);
        ::var::translate(sess, out, &[], ::util::pat_to_ident(&*arg.pat).unwrap(), &*arg.ty, None, true);
        write!(out, ";\n").unwrap();
        //writeln!(out, " {};", name.as_str()).unwrap();
    }

    match output {
        Some(ty) => {
            if stage == "fragment" {
                write!(out, "layout(location={}) ", mut_idx).unwrap();
            }
            write!(out, "out ").unwrap();
            ::ty::translate(sess, out, ty);
            write!(out, " {}", stage).unwrap();
            if let ast::TyVec(_) = ty.node {
                write!(out, "[]").unwrap();
            }
            write!(out, ";\n").unwrap();
            //::var::translate(sess, out, &[], ast::Ident::new(stage.to_string()), ty, None);
        }
        _ => ()
    }


    if let Some(output) = output {
    	::ty::translate(sess, out, output);
    }
    else {
    	write!(out, "void").unwrap();
    }
    write!(out, " {}(", name.as_str()).unwrap();
    // params
    /*for (i, &ast::Arg { ref ty, ref pat, ..}) in inputs.iter().enumerate() {
        if i != 0 {
            write!(out, ", ").unwrap();
        }
        ::var::translate(sess, out, &[], ::util::pat_to_ident(&*pat).unwrap(), &*ty, None, true);
    }*/
    /*if last_stage.is_some() {
        if inputs.len() > 0 {
            let ast::Arg { ref ty, ref pat, .. } = inputs[0];
            ::var::translate(sess, out, &[], ::util::pat_to_ident(&*pat).unwrap(), &*ty, None, true);
        }
    }*/
    write!(out, ") {{\n").unwrap();
    ::block::translate(sess, out, block, true);
    write!(out, "}}\n\n").unwrap();

    write!(out, "void main() {{\n").unwrap();
    if output.is_some() {
    	write!(out, "{} = ", stage).unwrap();
    }
    write!(out, "{}(", name.as_str()).unwrap();
    /*if let Some(last_stage) = last_stage {
        write!(out, "{}", last_stage).unwrap();
    }*/
    write!(out, ");\n}}\n").unwrap();

	/*write!(out, "void main() {{\n").unwrap();
	::block::translate(sess, out, block, true);
	write!(out, "}}\n\n").unwrap();*/
}