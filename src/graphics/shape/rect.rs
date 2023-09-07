use crate::{
    core::{index_buffer, shader, vertex_array, vertex_buffer, vertex_buffer_layout},
    math,
    util::{get_shader, ShaderProgram::Basic},
};

const DATA: &str = include_str!("../../../res/shaders/shape.color.shader");

pub struct Rect
{
    shader:  shader::Shader,
    va:      vertex_array::VertexArray,
    ib:      index_buffer::IndexBuffer,
    pub pos: math::Vec2,
}

impl Rect
{
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self
    {
        #[rustfmt::skip]
        let positions: [f32; 16] = [
             0., 0., 0., 0.,  // bottom left
             w, 0., 1., 0.,  // bottom right
             w, h, 1.0, 1.0, // top right
             0., h, 0., 1.// top left
        ];

        #[rustfmt::skip]
        let indices: [u32; 6] = [
            0, 1, 2, 
            2, 3, 0
        ];

        let vb = vertex_buffer::VertexBuffer::new(&positions);
        let mut va = vertex_array::VertexArray::new();
        let mut layout = vertex_buffer_layout::VertexBufferLayout::new();
        layout.push(2, gl::FLOAT);
        layout.push(2, gl::FLOAT);
        va.add_buffer(&vb, layout);

        let ib = index_buffer::IndexBuffer::new(&indices);

        let shader = shader::Shader::from_shader_string(get_shader(Basic));
        shader.bind();
        shader.set_uniform_f4("u_Color\0", 1., 0., 0., 1.);

        va.unbind();
        vb.unbind();
        ib.unbind();
        shader.unbind();

        let pos = math::vec2(x, y);

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
            *color.get_unchecked(idx)
        };
        self.shader.bind();
        self.shader.set_uniform_f4("u_Color\0", g(0), g(1), g(1), g(3));
    }
}

use crate::graphics::drawable::Drawable;
impl Drawable for Rect
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

    fn model(&self) -> glm::Mat4
    {
        let pos = glm::identity();
        glm::translate(&pos, &glm::vec3(pos[0], pos[1], 0.0))
    }

    fn texture(&self) -> Option<&crate::core::texture::Texture>
    {
        None
    }
}
