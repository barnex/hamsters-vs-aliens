/*

*/
#version 450 core

in  vec2 frag_tex_coord;
in vec3 frag_pos;
in vec3 frag_normal;
out vec4 output_color;

layout(location = 6, binding = 0) uniform sampler2D  tex;
layout(location = 7) uniform float inv_view_dist_sq = 1e-6;
layout(location = 8) uniform vec3 sun_dir = vec3(0.2, 0.8, 0.1);
layout(location = 10) uniform vec3 view_pos;
layout(location = 11) uniform vec3 fog_color;

void main() {
	vec3 normal = normalize(frag_normal);

	vec3 view_delta = frag_pos - view_pos;
	float dist = inv_view_dist_sq * dot(view_delta, view_delta);
	float fog = min(dist, 1.0);

	float costheta = dot(sun_dir, normal);
	float ambient = 0.5 * costheta + 0.7;
	float diffuse = max(0.0, costheta);
	float light = min(1.0, (0.3*diffuse + 0.8*ambient));
	vec3 view_dir = normalize(view_pos - frag_pos);
	vec3 refl_dir = reflect(normalize(-sun_dir), normal);
	float specular = pow(max(0.0, dot(view_dir, refl_dir)), 8);

	vec3 tex = texture(tex, frag_tex_coord).rgb;
	vec3 color = (1.0 - fog) * (light * tex + vec3(specular, specular, specular)) + vec3(fog * fog_color);
	output_color = vec4(color, 1.0);
}