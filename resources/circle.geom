#version 460

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

uniform mat4 projection;
uniform mat4 modelview;

void main() {
    vec4 center = modelview * vec4(gl_in[0].gl_Position.xyz, 1.0);

    // Emit the four corners of a square with the center as one corner
    gl_Position = projection * (center + vec4(-0.1, -0.1, 0.0, 0.0));
    EmitVertex();
    gl_Position = projection * (center + vec4(0.1, -0.1, 0.0, 0.0));
    EmitVertex();
    gl_Position = projection * (center + vec4(-0.1, 0.1, 0.0, 0.0));
    EmitVertex();
    gl_Position = projection * (center + vec4(0.1, 0.1, 0.0, 0.0));
    EmitVertex();

    EndPrimitive();
}