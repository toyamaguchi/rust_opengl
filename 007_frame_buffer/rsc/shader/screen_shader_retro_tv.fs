#version 140

in vec2 TexCoords;

uniform sampler2D uScreenTexture;
uniform float uScreenHeight;
uniform float uTime;

float noise1(float x)
{
    float result = 0.5 * sin(2*x) + 1.0;
    if (1.0 < result) {
        result = 1.0;
    } else if (result < 0.0) {
        result = 0.0;
    }
    return result;
}

float noise2(float x, float t)
{
    float result = 0.2*sin(x/10 - 2*t) + 1.0;
    if (1.0 < result) {
        result = 1.0;
    } else if (result < 0.0) {
        result = 0.0;
    }
    return result;
}

void main()
{
    float n1 = noise1(TexCoords.y*uScreenHeight);
    float n2 = noise2(TexCoords.y*uScreenHeight, uTime);
    gl_FragColor = vec4(texture(uScreenTexture, TexCoords).rgb*n1*n2, 1.0);
}
