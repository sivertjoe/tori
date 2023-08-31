use crate::core::{
    index_buffer::IndexBuffer, shader::Shader, util::gl_call, vertex_array::VertexArray,
};

pub struct Renderer {}

#[allow(dead_code)]
impl Renderer
{
    pub fn new() -> Self
    {
        Self {}
    }

    pub fn draw(&self, va: &VertexArray, ib: &IndexBuffer, shader: &Shader)
    {
        shader.bind();
        va.bind();
        ib.bind();

        unsafe
        {
            gl_call!(gl::DrawElements(
                gl::TRIANGLES,
                ib.count as _,
                gl::UNSIGNED_INT,
                std::ptr::null()
            ));
        }
    }

    pub fn clear(&self)
    {
        unsafe
        {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
