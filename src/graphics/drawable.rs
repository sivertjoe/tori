use crate::{core::*, math};
pub trait Drawable
{
    fn vertex_array(&self) -> &vertex_array::VertexArray;
    fn index_buffer(&self) -> &index_buffer::IndexBuffer;
    fn shader(&self) -> &shader::Shader;
    fn model(&self) -> math::Mat4;
    fn texture(&self) -> Option<&texture::Texture>;
}

impl<T> Drawable for &T
where
    T: Drawable,
{
    fn model(&self) -> math::Mat4
    {
        (*self).model()
    }

    fn shader(&self) -> &shader::Shader
    {
        (*self).shader()
    }

    fn vertex_array(&self) -> &vertex_array::VertexArray
    {
        (*self).vertex_array()
    }

    fn index_buffer(&self) -> &index_buffer::IndexBuffer
    {
        (*self).index_buffer()
    }

    fn texture(&self) -> Option<&texture::Texture>
    {
        (*self).texture()
    }
}
