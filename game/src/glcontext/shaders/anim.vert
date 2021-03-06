/*
	Vertex shader with:
	
	 	pitch
		+ translate + yaw
		+ translate
		+ project

	3D texture coordinate / color passthrough.

	TODO: it might be somewhat faster to fuse these into one matrix.
*/
#version 450 core

layout(location = 1) in vec3 vertex_pos;
layout(location = 2) in vec2 vertex_tex_coord;
layout(location = 5) in vec3 vertex_normal;

layout(location = 12) uniform vec3  int_translation;
layout(location = 9)  uniform float yaw_radians;
layout(location = 13) uniform float pitch_radians;
layout(location = 3)  uniform vec3  ext_translation;
layout(location = 4)  uniform mat4  proj;

out vec2 frag_tex_coord;
out vec3 frag_pos;
out vec3 frag_normal;

void main() {

	// pitch matrix
	float sp = sin(pitch_radians);
	float cp = cos(pitch_radians);
	mat3 pitch_matrix = mat3(
		1.0, 0.0, 0.0,
		0.0, cp, -sp,
		0.0, sp,  cp);

	// yaw matrix
	float sy = sin(yaw_radians);
	float cy = cos(yaw_radians);
	mat3 yaw_matrix = mat3(
		cy,  0.0, -sy,
		0.0, 1.0, 0.0,
		sy,  0.0, cy);

	vec3 model_pitched  = pitch_matrix * vertex_pos;
	vec3 model_yawed    = yaw_matrix * (model_pitched + int_translation);
	vec3 model_absolute = model_yawed + ext_translation;

	frag_pos       = model_absolute;
	frag_tex_coord = vertex_tex_coord;
	frag_normal    = yaw_matrix * (pitch_matrix * vertex_normal); // assumes orthonormal transform.
    gl_Position    = proj * vec4(model_absolute, 1.0);
}