// Request GLSL 4.0
#version 400

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_tex_coord;

out float frag_alpha;
out vec3 frag_position;
out vec3 frag_normal;
out vec2 frag_tex_coord;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;
uniform float uAlpha;

void main()
{
    frag_alpha = uAlpha;
    frag_position = vec3(uModel * vec4(in_position, 1.0));
    frag_normal = mat3(transpose(inverse(uModel))) * in_normal;
    frag_tex_coord = in_tex_coord;
    gl_Position = uProjection * uView * vec4(frag_position, 1.0);
}
