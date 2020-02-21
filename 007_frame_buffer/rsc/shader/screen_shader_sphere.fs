#version 140

in vec2 TexCoords;

uniform sampler2D uScreenTexture;

void main()
{
    gl_FragColor = vec4(texture(uScreenTexture, TexCoords).rgb, 1.0);
}
