#version 330

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;

smooth out vec4 theColor;

uniform mat4 cameraToClipMatrix;
uniform mat4 modelToCameraMatrix;

void main()
{
    vec4 cameraPos = modelToCameraMatrix * vec4(position, 1.0);
    gl_Position = cameraToClipMatrix * cameraPos;
    theColor = color;
}
