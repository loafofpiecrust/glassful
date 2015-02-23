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
layout(location=0) out Vertex vertex;
void vert() {
(vertex = Vertex((MATRIX * vec4(position, 1.0)), vec2(0, 0), vec4(1, 0.5, 1, 1), 1.2));
}

void main() {
vert();
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
layout(location=0) in Vertex geometry;
layout(location=0) out vec4 color;
void frag() {
return (color = vec4(vertex.position.x, 1.0, 0.5, 1.0));
}

void main() {
frag();
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
layout(location=0) in Vertex vertex[];
layout(location=0) out Vertex geometry;
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

