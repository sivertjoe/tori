use crate::{
    core::*,
    graphics::texture,
    math,
    util::{get_shader, ShaderProgram::Texture},
};
pub struct Sprite<'texture>
{
    va:     vertex_array::VertexArray,
    vb:     vertex_buffer::VertexBuffer,
    ib:     index_buffer::IndexBuffer,
    shader: shader::Shader,

    pos:     math::Mat4,
    size:    math::UVec2,
    texture: &'texture texture::Texture,
}

impl<'tex> Sprite<'tex>
{
    pub fn new(texture: &'tex texture::Texture) -> Self
    {
        let core = texture.get_core();
        let w = core.width as _;
        let h = core.height as _;
        #[rustfmt::skip]
        let positions: [f32; 16] = [
             0., 0., 0., 0., // bottom left
             w,  0., 1., 0., // bottom right
             w,  h,  1., 1., // top right
             0., h,  0., 1.  // top left
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

        let data = get_shader(Texture);
        let shader = shader::Shader::from_shader_string(data);
        shader.bind();

        // let texture = core::texture::Texture::new("res/textures/bird.png");
        // texture.bind(None);
        shader.set_uniform_1i("u_Texture\0", 0);

        va.unbind();
        vb.unbind();
        ib.unbind();
        shader.unbind();
        Self {
            va,
            vb,
            ib,
            shader,
            texture,

            pos: glm::translate(&glm::identity::<f32, 4>(), &glm::vec3(0., 0., 0.)),
            size: math::UVec2::new(w as _, h as _),
        }
    }

    pub fn set_pos(&mut self, x: isize, y: isize)
    {
        self.pos[12] = x as _;
        self.pos[13] = y as _;
    }

    pub fn set_size(&mut self, w: isize, h: isize)
    {
        let w = w as _;
        let h = h as _;
        #[rustfmt::skip]
        let positions: [f32; 16] = [
             0., 0., 0., 0., // bottom left
             w,  0., 1., 0., // bottom right
             w,  h,  1., 1., // top right
             0., h,  0., 1.  // top left
        ];
        let vb = vertex_buffer::VertexBuffer::new(&positions);
        let mut va = vertex_array::VertexArray::new();
        let mut layout = vertex_buffer_layout::VertexBufferLayout::new();
        layout.push(2, gl::FLOAT);
        layout.push(2, gl::FLOAT);
        va.add_buffer(&vb, layout);

        vb.unbind();
        va.unbind();
        self.vb = vb;
        self.va = va;
    }
}

use crate::graphics::drawable::Drawable;
impl<'t> Drawable for Sprite<'t>
{
    fn texture(&self) -> Option<&crate::core::texture::Texture>
    {
        Some(self.texture.get_core())
    }

    fn pos(&self) -> math::Mat4
    {
        self.pos
    }

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
}
