#shader vertex
#version 330 core
layout(location = 0) in vec4 position;

uniform mat4 u_MVP;

out vec2 fragCoord;

void main()
{
    gl_Position = u_MVP * position;
    fragCoord = position.xy;
}

#shader fragment
#version 330 core
layout(location = 0) out vec4 fragColor;

in vec2 fragCoord;
uniform vec4 u_Color;

void main()
{
    vec3 circleColor = vec3(0.85, 0.35, 0.2);

    float dist = length(fragCoord);
    if(dist < 0.495)
    {
        fragColor = vec4(circleColor, 1.0);
    } 
    else 
    {
        fragColor = vec4(0.0);
    }

}

