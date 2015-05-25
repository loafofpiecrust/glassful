#![deny(warnings)]

extern crate glassful;

use std::io;
use std::io::Read;

pub fn main() {
    let mut prog = String::new();
    io::stdin().read_to_string(&mut prog).unwrap();
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
