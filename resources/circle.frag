#version 460

out vec4 color;
uniform float scale;
uniform vec2 res;
in vec2 triCenter;
in vec3 obj_color;
in float radius;

void main() {
    vec2 ndc = 2.0 * gl_FragCoord.xy / res - 1.0;
    float dist = length(ndc-triCenter);

    if(dist*scale<radius) {
        color = vec4(obj_color,1.0);
    }else{
        discard;
    }
    //color = vec4(1-dist*scale, 0.0, 0.0, 1.0);
    //color = vec4(triCenter,0.0,1.0);
    //color = vec4(ndc,0.0,1.0);
    //color = vec4((ndc.x-triCenter.x)*100.0,0.0,0.0,1.0);
}