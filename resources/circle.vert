#version 460

in vec2 position;
uniform float scale;

void main() {
    float scaleFactor = 1.0/scale;
    gl_Position = vec4(position, 0.0, 1.0)*scaleFactor;
}