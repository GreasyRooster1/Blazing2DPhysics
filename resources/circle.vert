#version 460

in vec2 position;
in float radius;
in vec3 color;
uniform float scale;

out VS_OUT {
    float radius;
    vec3 color;
} vs_out;

void main() {
    float scaleFactor = 1.0/scale;
    gl_Position = vec4(position, 0.0, 1.0)*scaleFactor;
    vs_out.color = color;
    vs_out.radius = radius;
}