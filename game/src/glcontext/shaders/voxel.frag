/*

*/
#version 450 core

in  vec2 frag_tex_coord;
in  vec3 frag_light;
in  vec3 frag_pos;
out vec4 output_color;

layout(location = 6, binding = 0) uniform sampler2D  tex;
layout(location = 7) uniform float inv_view_dist_sq = 1e-6;
layout(location = 10) uniform vec3 view_pos;
layout(location = 11) uniform vec3 fog_color;

void main() {
	vec3 view_delta = frag_pos - view_pos;
	float dist = inv_view_dist_sq * dot(view_delta, view_delta);
	float fog = min(dist, 1.0);
	vec3 tex = texture(tex, frag_tex_coord).rgb;
	vec3 color = (1.0 - fog) * (tex * frag_light) + fog * fog_color;
	output_color = vec4(color, 1.0);
}