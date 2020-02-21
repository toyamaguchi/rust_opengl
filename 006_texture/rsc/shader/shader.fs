#version 140

struct Material {
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in float Alpha;
in vec3 FragPosition;
in vec3 Normal;
in vec2 TexCoords;

uniform sampler2D uScreenTexture;
uniform vec3 uViewPosition;
uniform Material uMaterial;
uniform Light uLight;

void main()
{
    // ambient
    vec3 ambient = uLight.ambient * texture(uScreenTexture, TexCoords).rgb;

    // diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(-uLight.direction);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = uLight.diffuse * diff * texture(uScreenTexture, TexCoords).rgb;

    // specular
    vec3 viewDir = normalize(uViewPosition - FragPosition);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), uMaterial.shininess);
    vec3 specular = uLight.specular * spec * uMaterial.specular;

    vec3 result = ambient + diffuse + specular;

    gl_FragColor = vec4(result, Alpha);
}
