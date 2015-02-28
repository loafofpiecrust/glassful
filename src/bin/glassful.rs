#![feature(old_io)]
#![deny(warnings)]

use std::old_io as io;

extern crate glassful;

pub fn main() {
    let prog = io::stdin().read_to_end().unwrap();
    let prog = String::from_utf8(prog).unwrap();
    let prog = glassful::translate(prog);
    if let Some(vert) = prog.vertex {
	    print!("// vertex\n{}\n", vert);
	}
	if let Some(frag) = prog.fragment {
    	print!("// fragment\n{}\n", frag);
	}
    if let Some(geom) = prog.geometry {
    	print!("// geometry\n{}\n", geom);
    }
}
