#version 450

uniform uvec2 iResolution;
uniform vec3 camPosition;
uniform vec2 camRotation;

out vec4 color;


vec2 isphere( in vec4 sph, in vec3 ro, in vec3 rd )
{
    vec3 oc = ro - sph.xyz;
    
	float b = dot(oc,rd);
	float c = dot(oc,oc) - sph.w*sph.w;
    float h = b*b - c;
    
    if( h<0.0 ) return vec2(-1.0);

    h = sqrt( h );

    return -b + vec2(-h,h);
} 

float mandelbulb(in vec3 p) {
    vec3 np = vec3(p.x, p.z, p.y);
    vec3 z = np;
	float dr = 1.0;
	float r = 0.0;

    float Power = 8.0;

	for (int i = 0; i < 10 ; i++) {
		r = length(z);

		if (r > 2) break;
		
		// convert to polar coordinates
		float theta = acos(z.z/r);
		float phi = atan(z.y,z.x);
		dr =  pow( r, Power-1.0)*Power*dr + 1.0;
		
		// scale and rotate the point
		float zr = pow( r,Power);
		theta = theta*Power;
		phi = phi*Power;
		
		// convert back to cartesian coordinates
		z = zr*vec3(sin(theta)*cos(phi), sin(phi)*sin(theta), cos(theta));
		z += np;
	}
	return 0.5*log(r)*r/dr;
}

float mandelbulb_fast(in vec3 p)
{
    vec3 w = p;
    float m = dot(w,w);

    vec4 trap = vec4(abs(w),m);
	float dz = 1.0;
    
    
	for( int i=0; i<10; i++ ) {
        float m2 = m*m;
        float m4 = m2*m2;
        dz = 8.0*sqrt(m4*m2*m)*dz + 1.0;
        
        float x = w.x; float x2 = x*x; float x4 = x2*x2;
        float y = w.y; float y2 = y*y; float y4 = y2*y2;
        float z = w.z; float z2 = z*z; float z4 = z2*z2;

        float k3 = x2 + z2;
        float k2 = inversesqrt( k3*k3*k3*k3*k3*k3*k3 );
        float k1 = x4 + y4 + z4 - 6.0*y2*z2 - 6.0*x2*y2 + 2.0*z2*x2;
        float k4 = x2 - y2 + z2;

        w.x = p.x +  64.0*x*y*z*(x2-z2)*k4*(x4-6.0*x2*z2+z4)*k1*k2;
        w.y = p.y + -16.0*y2*k3*k4*k4 + k1*k1;
        w.z = p.z +  -8.0*y*k4*(x4*x4 - 28.0*x4*x2*z2 + 70.0*x4*z4 - 28.0*x2*z2*z4 + z4*z4)*k1*k2;

        m = dot(w,w);
	
        if( m > 256.0 )
            break;
    }

    return 0.25*log(m)*sqrt(m)/dz;
}

vec4 lerp(vec4 a, vec4 b, float val) {
    return (1-val)*a + val*b;
}

float intersect(vec3 pos, vec3 ray, float detail, out float ops) {
    ops = 1;
    
    vec2 dis = isphere(vec4(0.0, 0.0, 0.0, 1.25), pos, ray);
    
    
    if (dis.y < 0.0) {
        return -1;
    } 

    dis.x = max(dis.x, 0);
    dis.y = min(dis.y, 10);

    float len = dis.x;
    float dist = dis.x; 

    for (int i = 0; i < 512; i++) {
        dist = mandelbulb(pos + ray*len);

        if (dist < 0.25*detail*len || len > dis.y) {
            ops = clamp(float(i) / 128.0, 0.0, 1); 
            break;
        }

        len += dist; 
    }

    if (len > dis.y) {
        return -1;
    } else {
        return len;
    }
}

void render(vec3 ray) {
    const float first_detail = 2.0/(float(iResolution.x)*1.5);
    
    vec4 col = vec4(0.2, 0.7, 0.3, 1.0);

    float ops;
    float l = intersect(camPosition, ray, first_detail, ops);

    if (l >= 0) 
        col = lerp(col, vec4(0.0, 0.0, 0.0, 1.0), sqrt(ops));
    else {
        color = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }


    color = col;
}

void main() {
    float yaw = (gl_FragCoord.x/iResolution.x - 0.5)*iResolution.x/iResolution.y + camRotation.x; 
    float pitch = -gl_FragCoord.y/iResolution.y + camRotation.y + 0.5;

    vec3 ray = vec3(
        sin(yaw)*cos(pitch),
        sin(pitch),
        cos(yaw)*cos(pitch));


    //color = vec4(ray, 1);


    render(ray);
}
