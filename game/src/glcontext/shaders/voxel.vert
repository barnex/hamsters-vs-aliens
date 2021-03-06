/*
	Vertex shader with translation (for instancing) + projection matrix.
	3D texture coordinate / color passthrough.
*/
#version 450 core

layout(location = 1) in vec3 vertex_pos;
layout(location = 2) in vec2 vertex_tex_coord;
layout(location = 4) uniform mat4 proj;
layout(location = 5) in vec3 vertex_light;

out vec2 frag_tex_coord;
out vec3 frag_light;
out vec3 frag_pos;

void main() {
	frag_pos       = vertex_pos;
	frag_tex_coord = vertex_tex_coord;
	frag_light     = vertex_light;
    gl_Position    = proj * vec4(vertex_pos, 1.0);
}