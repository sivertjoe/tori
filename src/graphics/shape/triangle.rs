use crate::{
    core::{index_buffer, shader, vertex_array, vertex_buffer, vertex_buffer_layout},
    math::{self, Vec2},
    util::{get_shader, ShaderProgram::Basic},
};

pub struct Triangle
{
    shader:  shader::Shader,
    va:      vertex_array::VertexArray,
    ib:      index_buffer::IndexBuffer,
    pub pos: Vec2,
}

impl Triangle
{
    pub fn new(p1: Vec2, p2: Vec2, p3: Vec2) -> Self
    {
        // SAFETY:
        // points cannot be created without calling
        // something like `new`.
        let g = |p: Vec2, idx: usize| unsafe
        {
            *p.get_unchecked(idx)
        };

        #[rustfmt::skip]
        let positions: [f32; 6] = [
            g(p1, 0), g(p1, 1),
            g(p2, 0), g(p2, 1),
            g(p3, 0), g(p3, 1),
        ];

        #[rustfmt::skip]
        let indices: [u32; 3] = [
            0, 1, 2, 
        ];

        let vb = vertex_buffer::VertexBuffer::new(&positions);
        let mut va = vertex_array::VertexArray::new();
        let mut layout = vertex_buffer_layout::VertexBufferLayout::new();
        layout.push(2, gl::FLOAT);
        va.add_buffer(&vb, layout);

        let shader = shader::Shader::from_shader_string(get_shader(Basic));
        shader.bind();
        shader.set_uniform_f4("u_Color\0", 1., 0., 0., 1.0);

        let ib = index_buffer::IndexBuffer::new(&indices);

        let pos = Vec2::new(0.0, 0.0);

        va.unbind();
        vb.unbind();
        ib.unbind();
        shader.unbind();

        Self {
            shader,
            va,
            ib,
            pos,
        }
    }

    pub fn set_color(&mut self, color: math::Vec4)
    {
        let g = |idx: usize| unsafe
        {
            *color.get_unchecked(idx) as f32
        };
        self.shader.bind();
        self.shader.set_uniform_f4("u_Color\0", g(0), g(1), g(1), g(3));
    }
}

use crate::graphics::drawable::Drawable;
impl Drawable for Triangle
{
    fn shader(&self) -> &shader::Shader
    {
        &self.shader
    }

    fn vertex_array(&self) -> &vertex_array::VertexArray
    {
        &self.va
    }

    fn index_buffer(&self) -> &index_buffer::IndexBuffer
    {
        &self.ib
    }

    fn pos(&self) -> glm::Mat4
    {
        let pos = glm::identity();
        glm::translate(&pos, &glm::vec3(pos[0], pos[1], 0.0))
    }

    fn texture(&self) -> Option<&crate::core::texture::Texture>
    {
        None
    }
}
