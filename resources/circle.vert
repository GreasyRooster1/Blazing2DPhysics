#version 460

in vec2 position;

void main() {
    float scaleFactor = 1/1000;
    gl_Position = vec4(position, 0.0, 1.0)*scaleFactor;
}