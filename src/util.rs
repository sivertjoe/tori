const SHADERS: [&str; 4] = [
    include_str!("../res/shaders/shape.color.shader"),
    include_str!("../res/shaders/shape.texture.shader"),
    include_str!("../res/shaders/text.shader"),
    include_str!("../res/shaders/circle.shader"),
];

pub enum ShaderProgram
{
    Basic,
    Texture,
    Text,
    Circle,
}

pub fn get_shader(program: ShaderProgram) -> &'static str
{
    use ShaderProgram as SP;
    match program
    {
        SP::Basic => SHADERS[0],
        SP::Texture => SHADERS[1],
        SP::Text => SHADERS[2],
        SP::Circle => SHADERS[3],
    }
}
