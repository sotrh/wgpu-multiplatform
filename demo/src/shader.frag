#version 450

layout(location=0) in vec3 clipCoords;
layout(location=0) out vec4 fragColor;

#define MAX_STEPS 100
#define MAX_DIST 100.
#define SURF_DIST .001

float sphere(vec3 p, vec3 c, float r) {
    return length(c - p) - r;
}

float scene(vec3 p) {
    float dSphere = sphere(p, vec3(0, 1, 0), 1);
    float dPlane = p.y;
    float d = min(dPlane, dSphere);
    return d;
}

float rayMarch(vec3 ro, vec3 rd) {
    float marchDist = 0;
    for (int i=0; i<MAX_STEPS; i++) {
        vec3 p = ro + rd * marchDist;
        float distToSurface = scene(p);
        marchDist += distToSurface;
        if (marchDist > MAX_DIST || distToSurface < SURF_DIST) break; 
    }

    return marchDist;
}

vec3 getNormal(vec3 p) {
    vec2 e = vec2(0.01, 0);
    float d = scene(p);
    vec3 n = vec3(
        d - scene(p - e.xyy),
        d - scene(p - e.yxy),
        d - scene(p - e.yyx)
    );
    return normalize(n);
}

float getLight(vec3 p, vec3 rd) {
    // Diffuse
    vec3 lightPos = vec3(2, 2, -4);
    vec3 l = normalize(lightPos - p);
    vec3 n = getNormal(p);
    float diffuse = max(dot(n, l), 0.0);

    // Specular
    vec3 half_dir = normalize(rd + l);
    float spec = pow(max(dot(n, half_dir), 0.0), 32);

    // Shadow
    float d = rayMarch(p + n * SURF_DIST * 2.0, l);

    float light = diffuse + spec;

    if (d < length(lightPos - p)) {
        light *= 0.1;
    }

    return light;
}

void main() {
    vec3 ro = vec3(0, 1, -3);
    vec3 rd = normalize(vec3(clipCoords.x, clipCoords.y, 1));

    float d = rayMarch(ro, rd);

    vec3 p = ro + rd * d;
    float light = getLight(p, rd);

    vec3 col = vec3(light);
    
    fragColor = vec4(col, 1.0);
}