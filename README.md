# Rust-like syntax for GLSL

[![Build Status](https://travis-ci.org/kmcallister/glassful.svg?branch=master)](https://travis-ci.org/kmcallister/glassful)

glassful translates a small subset of Rust to [OpenGL Shading Language][].

Besides one's personal preferences regarding Rust-like vs. C-like syntax, this
has a few specific advantages for the Rust programmer:

* Syntax is checked at Rust compile time, with friendly rustc-style errors
* There's less cognitive overhead switching between CPU and GPU code
* Shaders can use Rust macros!
* Shaders embedded in a Rust program have syntax highlighting

The library is still in a *very* early stage! Many improvements are possible.
See the [issue tracker][] and don't hesitate to send pull requests :)

[OpenGL Shading Language]: https://www.opengl.org/documentation/glsl/
[issue tracker]: https://github.com/kmcallister/glassful/issues

## Function syntax

My changes allow for the consolodation of shaders into one file or string,
with each stage as a separate function. Each is marked with an attribute of it's
stage name, but the function itself can be called anything.
eg. the vertex stage could be:
```rs
#[vertex]
fn vert_to_color(position: Vec3) -> Vec3 {
    ..
}
```
The geometry stage is optional of course.

## Values

Normal function parameters become `in` values in the order that they are
written.
Parameters marked with `mut` get turned into `out` values
and are write-only. If there is a return value on the function,
it gets turned into the very last `out` value.

Statics are uniforms by default, and if you want to use other kinds, eg. varying or attributes,
as statics, you must split the shaders into multiple invokations of glassful.
This is because all content is copied into each glsl output shader, including all
functions, uniforms, consts structs, etc., except for the functions that are marked as stage `main`s
with the stage name attribute.

## Structs

Unfortunately, struct init syntax is currently limited to how glsl inits structs,
with the members unnamed and in order, eg:
```rs
struct Vertex {
    pos: Vec4,
    color: Vec3,
}

#[vertex]
fn vert(pos: Vec3) -> Vertex {
    Vertex(Vec4(pos, 1.0), Vec3(0.0, 1.0, 0.4))
}
```

## Usage

There are three ways to invoke the translator.  The language syntax is exactly
the same in all three cases.

### As a macro

```rust
#![plugin(glassful_macros)]
#[macro_use] extern crate glassful_macros;

const PROGRAM: (&'static str, &'static str) = glassful! {
    #![version="330"]

    const FAV_COLOR: Vec3 = Vec3(0.4, 0.2, 0.6);
    static MATRIX: Mat4 = UNINIT;

    #[vertex]
    fn vert(position: Vec2, mut color: Vec3) {
        gl_Position = MATRIX * Vec4(position, 0.0, 1.0);
        color = Vec3(0.5*(position + Vec2(1.0, 1.0)), 0.0);
    }

    #[fragment]
    fn frag(color: Vec3) {
        gl_FragColor = Vec4(color, 1.0);
    }
};

let program = glium::Program::from_source(&display, PROGRAM.0, PROGRAM.1, None);
```

See `examples/gradient/` for a full glium/glutin example.

~! See `examples/random/` for a glsl 330 + parameter syntax example.

### As an external program

```
$ ./target/glassful < shader.glassful > shader.glsl
```

### As an ordinary library

```rust
extern crate glassful;

pub fn main() {
    let prog = io::stdin().read_to_end().unwrap();
    let prog = String::from_utf8(prog).unwrap();
    let (vert, frag, geom) = glassful::translate(prog);
    print!("// vert\n{}\n", vert);
    print!("// frag\n{}\n", frag);
    if let Some(geom) = geom {
        print!("// geom\n{}\n", geom);
    }
}
```
