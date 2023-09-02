use crate::{
    core::{index_buffer, shader, vertex_array, vertex_buffer, vertex_buffer_layout},
    math,
};

const DATA: &'static str = include_str!("../../../res/shaders/shape.color.shader");

pub struct Rect
{
    shader: shader::Shader,
    va:     vertex_array::VertexArray,
    ib:     index_buffer::IndexBuffer,
    pos:    math::Mat4,
}

impl Rect
{
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self
    {
        let x: f32 = x as _;
        let y: f32 = y as _;
        let w: f32 = w as _;
        let h: f32 = h as _;
        #[rustfmt::skip]
        let positions: [f32; 16] = [
             x, y, 0., 0.,  // bottom left
             x + w, y, 1., 0.,  // bottom right
             x + w, y + h, 1.0, 1.0, // top right
             x, y + h, 0., 1.// top left
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

        let shader = shader::Shader::from_shader_string(DATA);
        shader.bind();
        shader.set_uniform_f4("u_Color\0", 1., 0., 0., 1.);

        va.unbind();
        vb.unbind();
        ib.unbind();
        shader.unbind();

        let pos = glm::translate(&glm::identity::<f32, 4>(), &glm::vec3(0., 0., 0.));

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

    fn pos(&self) -> glm::Mat4
    {
        self.pos.clone()
    }
}