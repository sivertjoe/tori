const SHADERS: [&'static str; 1] = [include_str!("../res/shaders/shape.color.shader")];

pub enum ShaderProgram
{
    Basic,
}

pub fn get_shader(program: ShaderProgram) -> &'static str
{
    use ShaderProgram as SP;
    match program
    {
        SP::Basic => SHADERS[0],
    }
}
