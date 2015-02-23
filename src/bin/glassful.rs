#![feature(old_io)]
#![deny(warnings)]

use std::old_io as io;

extern crate glassful;

pub fn main() {
    let prog = io::stdin().read_to_end().unwrap();
    let prog = String::from_utf8(prog).unwrap();
    let (vert, frag, geom) = glassful::translate(prog);
    print!("// vertex\n{}\n", vert);
    print!("// fragment\n{}\n", frag);
    if let Some(geom) = geom {
    	print!("// geometry\n{}\n", geom);
    }
}
