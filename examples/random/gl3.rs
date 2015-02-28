#![version="330"]

static MATRIX: Mat4 = UNINIT;
static PROJECTION: Mat4 = UNINIT;

struct Vertex {
	position: Pnt4,
	tex_coords: Vec2,
	color: Vec4,
	size: f32,
}


#[vertex]
fn vert(
	// in
	position: Pnt3,
	normal: Vec3,
) -> Vertex {
	Vertex(
		Matrix * Pnt4(position, 1.0),
		Vec2(0, 0),
		Vec4(1, 0.5, 1, 1),
		1.2,
	)
}

#[geometry(points, triangle_strip="4")]
fn geom(vertex: [Vertex], mut geometry: Vertex) {
	let size: f32 = vertex[0].size;
	let right: Vec3 = Vec3(1,0,0) * size;
	let up: Vec3 = Vec3(0,1,0) * size;
	let pos: Vec3 = Vec4(view * vertex[0].position).xyz;

	geometry.color = vertex[0].color;

	geometry.position = PROJECTION * Pnt4(pos - right - up, 1.0);
	geometry.tex_coords = Vec2(0, 0);
	EmitVertex();
}

#[fragment]
fn frag(geometry: Vertex) -> Vec4 {
	Vec4(geometry.position.x, 1.0, 0.5, 1.0)
}