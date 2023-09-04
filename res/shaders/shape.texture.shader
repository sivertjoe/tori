#shader vertex
#version 330 core
layout(location = 0) in vec4 position;
layout(location = 1) in vec2 texCoord;

out vec2 v_TexCoord;

uniform mat4 u_MVP;

void main()
{
    gl_Position = u_MVP * position;
    v_TexCoord = texCoord;
}

#shader fragment
#version 400 core // hm, consts are not supported in 330?
layout(location = 0) out vec4 color;

in vec2 v_TexCoord;

uniform sampler2D u_Texture;

uniform uint u_Idx = 0;
uniform float u_Cols = 1.0;
uniform float u_Rows   = 1.0;
uniform uint u_Num_Sprites = 1;

void main()
{
    uint sprite_idx = u_Idx % u_Num_Sprites;
    vec2 pos = vec2(sprite_idx % int(u_Cols), int(sprite_idx / u_Cols));
    
    vec4 fragColor = texture(u_Texture, vec2((v_TexCoord.x / u_Cols) + pos.x * (1.0 / u_Cols),
        (v_TexCoord.y / u_Rows) + pos.y * (1.0 / u_Rows)));
    color = fragColor;
}
