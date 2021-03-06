/*
	Vertex shader with translation (for instancing) + projection matrix.
	3D texture coordinate / color passthrough.
*/
#version 450 core

layout(location = 1) in vec3 vertex_pos;
layout(location = 5) in vec4 vertex_color;
layout(location = 4) uniform mat4 proj;

out vec3 frag_color;

void main() {
	frag_color     = vertex_color;
    gl_Position    = proj * vec4(vertex_pos, 1.0);
}