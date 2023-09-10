use crate::{
    core::{shader, vertex_array, vertex_buffer, vertex_buffer_layout},
    util::{get_shader, ShaderProgram::Text},
};
pub struct Quad
{
    pub va:     vertex_array::VertexArray,
    pub vb:     vertex_buffer::VertexBuffer,
    pub shader: shader::Shader,
}

impl Quad
{
    pub fn new() -> Self
    {
        let vb =
            vertex_buffer::VertexBuffer::new_dynamic(6 * 4 * std::mem::size_of::<f32>() as isize);
        let mut va = vertex_array::VertexArray::new();
        let mut layout = vertex_buffer_layout::VertexBufferLayout::new();
        layout.push(4, gl::FLOAT);
        va.add_buffer(&vb, layout);

        let shader = shader::Shader::from_shader_string(get_shader(Text));

        vb.unbind();
        va.unbind();
        shader.unbind();

        Self {
            va,
            vb,
            shader,
        }
    }
}
