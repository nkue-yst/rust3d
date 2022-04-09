// Request GLSL 4.0
#version 400

in vec3 inPosition;

out vec3 fragPosition;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

void main()
{
    fragPosition = vec3(uModel * vec4(inPosition, 1.0));
    gl_Position = uProjection * uView * vec4(fragPosition, 1.0);
}
