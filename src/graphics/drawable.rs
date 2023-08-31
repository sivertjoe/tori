use crate::core::*;
pub trait Drawable
{
    fn vertex_array(&self) -> vertex_array::VertexArray;
    fn index_buffer(&self) -> index_buffer::IndexBuffer;
    fn shader(&self) -> shader::Shader;
}
