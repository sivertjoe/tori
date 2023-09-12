use crate::{
    core::*,
    math,
    util::{get_shader, ShaderProgram},
};

pub struct Circle
{
    va:         vertex_array::VertexArray,
    vb:         vertex_buffer::VertexBuffer,
    ib:         index_buffer::IndexBuffer,
    shader:     shader::Shader,
    pub center: math::Vec2,
    pub radius: f32,
}

impl Circle
{
    pub fn new(center: math::Vec2, radius: f32, color: math::Vec4) -> Self
    {
        #[rustfmt::skip]
        let quad: [f32; 16] = [
            -0.5, -0.5,  0., 0., // bottom left
             0.5, -0.5,  1., 0., // bottom right
             0.5,  0.5,  1., 1., // top right
            -0.5,  0.5,  0., 1.  // top left
        ];

        #[rustfmt::skip]
        let indices: [u32; 6] = [
            0, 1, 2, 
            2, 3, 0
        ];

        let vb = vertex_buffer::VertexBuffer::new(&quad);
        let mut va = vertex_array::VertexArray::new();
        let mut layout = vertex_buffer_layout::VertexBufferLayout::new();
        layout.push(2, gl::FLOAT);
        layout.push(2, gl::FLOAT);
        va.add_buffer(&vb, layout);

        let ib = index_buffer::IndexBuffer::new(&indices);

        let shader = shader::Shader::from_shader_string(get_shader(ShaderProgram::Circle));
        shader.bind();
        shader.set_uniform_f4("u_Color\0", color[0], color[1], color[2], color[3]);

        va.unbind();
        vb.unbind();
        ib.unbind();
        shader.unbind();

        Self {
            va,
            vb,
            ib,
            shader,
            center,
            radius,
        }
    }
}

use crate::graphics::drawable::{std_draw, Drawable};
impl Drawable for Circle
{
    fn draw(&self, proj: math::Mat4)
    {
        self.shader.bind();

        let mut model = math::identity();
        model = math::translate(&model, &math::vec3(self.center[0], self.center[1], 1.0));
        model = math::scale(&model, &math::vec3(self.radius, self.radius, 1.0));

        std_draw(&self.va, &self.ib, &self.shader, model, proj, None);
    }
}
