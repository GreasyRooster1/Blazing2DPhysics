#version 460

layout(points) in;
layout(triangle_strip, max_vertices = 3) out;
uniform float scale;

out vec2 triCenter;
out vec3 obj_color;

in VS_OUT {
    float radius;
    vec3 color;
} gs_in[];

void main() {
    float scaleFactor = 1.0/scale * gs_in[0].radius;
    vec4 center = vec4(gl_in[0].gl_Position.xyz, 1.0);

    triCenter = center.xy;
    obj_color = gs_in[0].color;

    // Emit the tri verticies
    gl_Position = (center + vec4(0.0, 2.0,0.0,0.0)* scaleFactor) ;
    EmitVertex();
    gl_Position = (center + vec4(1.7312, -1.0,0.0,0.0)* scaleFactor) ;  //sqrt(3)
    EmitVertex();
    gl_Position = (center + vec4(-1.7312, -1.0,0.0,0.0)* scaleFactor);
    EmitVertex();

    EndPrimitive();
}