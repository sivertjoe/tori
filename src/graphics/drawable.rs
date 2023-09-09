use crate::math;
pub trait Drawable
{
    fn draw(&self, proj: math::Mat4);
}

impl<T> Drawable for &T
where
    T: Drawable,
{
    fn draw(&self, proj: math::Mat4)
    {
        (*self).draw(proj);
    }
}

use crate::core::{
    index_buffer::IndexBuffer, shader::Shader, texture::Texture, vertex_array::VertexArray,
};
pub(crate) fn std_draw(
    va: &VertexArray,
    ib: &IndexBuffer,
    shader: &Shader,
    model: math::Mat4,
    proj: math::Mat4,
    texture: Option<&Texture>,
)
{
    if let Some(t) = texture
    {
        t.bind(None);
    }

    shader.bind();

    let mvp = proj * model;
    shader.set_uniform_mat4f("u_MVP\0", &mvp);

    shader.bind();
    va.bind();
    ib.bind();

    unsafe
    {
        gl::DrawElements(gl::TRIANGLES, ib.count as _, gl::UNSIGNED_INT, std::ptr::null());
    }

    if let Some(t) = texture
    {
        t.unbind();
    }
}
