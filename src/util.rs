const SHADERS: [&str; 3] = [
    include_str!("../res/shaders/shape.color.shader"),
    include_str!("../res/shaders/shape.texture.shader"),
    include_str!("../res/shaders/text.shader"),
];

pub enum ShaderProgram
{
    Basic,
    Texture,
    Text,
}

pub fn get_shader(program: ShaderProgram) -> &'static str
{
    use ShaderProgram as SP;
    match program
    {
        SP::Basic => SHADERS[0],
        SP::Texture => SHADERS[1],
        SP::Text => SHADERS[2],
    }
}
