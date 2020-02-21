#version 140

in vec3 iPosition;
in vec2 iTexCoords;

out vec2 TexCoords;

void main()
{
    TexCoords = iTexCoords;
    gl_Position = vec4(iPosition, 1.0);
}
