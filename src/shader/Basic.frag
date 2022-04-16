// Request GLSL 4.0
#version 400

struct Material {
    float shininess;
    vec3 specular;
};

struct Light {
    vec3 ambient;
    vec3 diffuse;
    vec3 direction;
    vec3 specular;
};

in float frag_alpha;
in vec3 frag_position;
in vec3 frag_normal;
in vec2 frag_tex_coord;

out vec4 final_color;

uniform Light uLight;
uniform Material uMaterial;
uniform sampler2D uScreenTexture;
uniform vec3 uViewPosition;

void main()
{
    vec3 ambient = uLight.ambient * texture(uScreenTexture, frag_tex_coord).rgb;

    vec3 normal = normalize(frag_normal);
    vec3 light_direction = normalize(-uLight.direction);
    float diff = max(dot(normal, light_direction), 0.0);
    vec3 diffuse = uLight.diffuse * diff * texture(uScreenTexture, frag_tex_coord).rgb;

    vec3 view_direction = normalize(uViewPosition - frag_position);
    vec3 reflect_direction = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), uMaterial.shininess);
    vec3 specular = uLight.specular * spec * uMaterial.specular;

    vec3 result = ambient + diffuse + specular;

    final_color = vec4(result, frag_alpha);
}
