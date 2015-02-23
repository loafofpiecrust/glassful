use std::fmt::Write;
use syntax::ast;
use syntax::parse::ParseSess;

pub struct Shaders {
    pub template: String,
    pub vertex: Option<String>,
    pub fragment: Option<String>,
    pub geometry: Option<String>,
}

impl Shaders {
	pub fn new(out: String) -> Self {
		Shaders {
			template: out,
			vertex: None,
			fragment: None,
			geometry: None,
		}
	}

	pub fn vertex(&self) -> String {
		match self.vertex {
			Some(ref sh) => self.template.clone() + &sh[..],
			None => String::new()
		}
	}

	pub fn fragment(&self) -> String {
		match self.fragment {
			Some(ref sh) => self.template.clone() + &sh[..],
			None => String::new()
		}
	}

	pub fn geometry(&self) -> Option<String> {
		self.geometry.clone().map(|sh| self.template.clone() + &sh[..])
	}
}

pub fn translate(sess: &ParseSess,
				 out: &mut String,
				 name: ast::Ident,
				 stage: &'static str,
				 inputs: &[ast::Arg],
				 output: Option<&ast::Ty>,
				 block: &ast::Block,
				) {
    //let diag = &sess.span_diagnostic;

    let mut idx = 0;
    let mut mut_idx = 0;
    for arg in inputs.iter() {
        /*if i != 0 {
            write!(out, ", ").unwrap();
        }*/

        match arg.pat.node.clone() {
        	ast::PatIdent(ast::BindByValue(ast::MutMutable), _, _) => {
        		write!(out, "layout(location={}) out ", mut_idx).unwrap();
        		mut_idx += 1;
        	}
        	_ => {
		        write!(out, "layout(location={}) in ", idx).unwrap();
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
    		write!(out, "layout(location={}) out ", mut_idx).unwrap();
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
    write!(out, ") {{\n").unwrap();
    ::block::translate(sess, out, block, true);
    write!(out, "}}\n\n").unwrap();

    write!(out, "void main() {{\n").unwrap();
    if output.is_some() {
    	write!(out, "{} = ", stage).unwrap();
    }
    write!(out, "{}();\n", name.as_str()).unwrap();
    write!(out, "}}\n").unwrap();

	/*write!(out, "void main() {{\n").unwrap();
	::block::translate(sess, out, block, true);
	write!(out, "}}\n\n").unwrap();*/
}