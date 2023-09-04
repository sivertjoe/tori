const SHADERS: [&'static str; 2] = [
    include_str!("../res/shaders/shape.color.shader"),
    include_str!("../res/shaders/shape.texture.shader"),
];

pub enum ShaderProgram
{
    Basic,
    Texture,
}

pub fn get_shader(program: ShaderProgram) -> &'static str
{
    use ShaderProgram as SP;
    match program
    {
        SP::Basic => SHADERS[0],
        SP::Texture => SHADERS[1],
    }
}
