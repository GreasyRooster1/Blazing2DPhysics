#version 460

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;
uniform float scale;

void main() {
    float scaleFactor = 1.0/scale;
    vec4 center = vec4(gl_in[0].gl_Position.xyz, 1.0);

    // Emit the four corners of a square with the center as one corner
    gl_Position = (center + vec4(0.0, 2.0,0.0,0.0)) * scaleFactor;
    EmitVertex();
    gl_Position = (center + vec4(1.7312, -1.0,0.0,0.0)) * scaleFactor;  //sqrt(3)
    EmitVertex();
    gl_Position = (center + vec4(-1.7312, -1.0,0.0,0.0)) * scaleFactor;
    EmitVertex();

    EndPrimitive();
}