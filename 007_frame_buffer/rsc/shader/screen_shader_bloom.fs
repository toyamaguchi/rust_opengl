#version 140

in vec2 TexCoords;

uniform sampler2D uScreenTexture;

void main()
{
    vec2 tex_offset = 1.0/textureSize(uScreenTexture, 0);
    float ratio[6] = float[] (0.398942, 0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);
    vec3 original_color = ratio[0]*texture(uScreenTexture, TexCoords).rgb;
    vec3 color = vec3(0, 0, 0);
    for (int x = 1; x < 6; x++) {
        for (int y = 1; y < 6; y++) {
            color += ratio[x]*ratio[y]*texture(
                uScreenTexture, TexCoords + vec2(tex_offset.x*x, tex_offset.y*y)).rgb;
            color += ratio[x]*ratio[y]*texture(
                uScreenTexture, TexCoords + vec2(-tex_offset.x*x, tex_offset.y*y)).rgb;
            color += ratio[x]*ratio[y]*texture(
                uScreenTexture, TexCoords + vec2(tex_offset.x*x, -tex_offset.y*y)).rgb;
            color += ratio[x]*ratio[y]*texture(
                uScreenTexture, TexCoords + vec2(-tex_offset.x*x, -tex_offset.y*y)).rgb;
        }
    }
    gl_FragColor = vec4(original_color + color, 1.0);
}
