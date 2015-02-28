input type: Mat4
input type: Mat4
input type: Pnt4
input type: Vec2
input type: Vec4
input type: f32
output type: float
input type: Pnt3
input type: Vec3
input type: Vertex
output type: Vertex
input type: Vertex
output type: Vertex
input type: Vertex
output type: Vertex
input type: Matrix
input type: Pnt4
input type: position
output type: position
input type: Vec2
input type: Vec4
input type: Vertex
output type: Vertex
input type: Vertex
output type: Vertex
input type: f32
output type: float
input type: vertex
output type: vertex
input type: Vec3
input type: Vec3
input type: size
output type: size
input type: Vec3
input type: Vec3
input type: size
output type: size
input type: Vec3
input type: Vec4
input type: view
output type: view
input type: vertex
output type: vertex
input type: geometry
output type: geometry
input type: vertex
output type: vertex
input type: geometry
output type: geometry
input type: PROJECTION
output type: PROJECTION
input type: Pnt4
input type: pos
output type: pos
input type: right
output type: right
input type: up
output type: up
input type: geometry
output type: geometry
input type: Vec2
input type: EmitVertex
output type: EmitVertex
input type: Vertex
output type: Vertex
input type: Vec4
input type: Vec4
input type: Vec4
input type: geometry
output type: geometry
// vertex
#version 330

uniform mat4 MATRIX;
uniform mat4 PROJECTION;
struct Vertex {
	vec4 position;
	vec2 tex_coords;
	vec4 color;
	float size;
};
layout(location=0) in vec3 position;
layout(location=1) in vec3 normal;
out Vertex vertex;
Vertex vert() {
return Vertex((matrix * vec4(position, 1.0)), vec2(0, 0), vec4(1, 0.5, 1, 1), 1.2);
}

void main() {
vertex = vert();
}

// fragment
#version 330

uniform mat4 MATRIX;
uniform mat4 PROJECTION;
struct Vertex {
	vec4 position;
	vec2 tex_coords;
	vec4 color;
	float size;
};
in Vertex geometry;
layout(location=0) out vec4 fragment;
vec4 frag() {
return vec4(geometry.position.x, 1.0, 0.5, 1.0);
}

void main() {
fragment = frag();
}

// geometry
#version 330

uniform mat4 MATRIX;
uniform mat4 PROJECTION;
struct Vertex {
	vec4 position;
	vec2 tex_coords;
	vec4 color;
	float size;
};
layout(points) in;
layout(triangle_strip, max_vertices = 4) out;
in Vertex vertex[];
out Vertex geometry;
void geom() {
const float size = vertex[0].size;
const vec3 right = (vec3(1, 0, 0) * size);
const vec3 up = (vec3(0, 1, 0) * size);
const vec3 pos = vec4((view * vertex[0].position)).xyz;
(geometry.color = vertex[0].color);
(geometry.position = (PROJECTION * vec4(((pos - right) - up), 1.0)));
(geometry.tex_coords = vec2(0, 0));
EmitVertex();
}

void main() {
geom();
}

