#![version="330"]

static MATRIX: Mat4 = UNINIT;
static PROJECTION: Mat4 = UNINIT;

struct Vertex {
	position: Pnt4,
	tex_coords: Vec2,
	color: Vec4,
	size: f32,
	texture: Sampler2D<i32>,
}


#[vertex]
fn vert(
	// in
	position: Pnt3<f64>,
	normal: Vec3,
) -> Vertex {
	Vertex(
		MATRIX * pnt4(position, 1.0),
		vec2(0, 0),
		vec4(1, 0.5, 1, 1),
		1.2,
	)
}

#[geometry(points, triangle_strip="4")]
fn geom(vertex: [Vertex], mut geometry: Vertex) {
	let size: f32 = vertex[0].size;
	let right: Vec3 = vec3(1,0,0) * size;
	let up: Vec3 = vec3(0,1,0) * size;
	let pos: Vec3 = vec4(view * vertex[0].position).xyz;

	geometry.color = vertex[0].color;

	geometry.position = PROJECTION * pnt4(pos - right - up, 1.0);
	geometry.tex_coords = vec2(0, 0);
	EmitVertex();
}

#[fragment]
fn frag(geometry: Vertex) -> Vec4 {
	vec4(geometry.position.x, 1.0, 0.5, 1.0)
}
