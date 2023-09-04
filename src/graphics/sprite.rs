use std::rc::Rc;

use crate::{
    core::*,
    graphics::{position::Position, texture},
    math,
    util::{get_shader, ShaderProgram::Texture},
};
pub struct Sprite<'texture>
{
    va:     vertex_array::VertexArray,
    vb:     vertex_buffer::VertexBuffer,
    ib:     index_buffer::IndexBuffer,
    shader: Rc<shader::Shader>,

    pub pos: Position,
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

        shader.set_uniform_1i("u_Texture\0", 0);

        shader.set_uniform_1u("u_Idx", 0);
        shader.set_uniform_1u("u_Num_Sprites", 1);
        shader.set_uniform_1f("u_Cols", 1.0);
        shader.set_uniform_1f("u_Rows", 1.0);

        va.unbind();
        vb.unbind();
        ib.unbind();
        shader.unbind();
        Self {
            va,
            vb,
            ib,
            shader: Rc::new(shader),
            texture,

            pos: Position::new(math::DVec::new(0, 0)),
            size: math::UVec2::new(w as _, h as _),
        }
    }

    pub fn get_size(&self) -> math::UVec2
    {
        self.size
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

        self.size[0] = w as _;
        self.size[1] = h as _;
    }

    pub fn make_sprite_sheet(&mut self, num_cols: u32, num_rows: u32) -> SpriteSheet
    {
        let shader = &self.shader;
        shader.bind();
        shader.set_uniform_1u("u_Idx", 0);
        shader.set_uniform_1u("u_Num_Sprites", num_cols * num_rows);
        shader.set_uniform_1f("u_Cols", num_cols as _);
        shader.set_uniform_1f("u_Rows", num_rows as _);
        shader.unbind();


        let w = self.size[0] / num_cols;
        let h = self.size[1] / num_rows;

        self.set_size(w as _, h as _);

        SpriteSheet {
            shader: Rc::clone(&self.shader),
            num_cols,
            num_rows,
            idx: 0,
        }
    }
}

pub struct SpriteSheet
{
    shader:   Rc<shader::Shader>,
    num_cols: u32,
    num_rows: u32,
    idx:      u32,
}

impl SpriteSheet
{
    pub fn get_num_cols(&self) -> u32
    {
        self.num_cols
    }

    pub fn get_num_rows(&self) -> u32
    {
        self.num_cols
    }

    pub fn get_idx(&self) -> u32
    {
        self.idx
    }

    pub fn set_idx(&mut self, idx: u32)
    {
        self.shader.bind();
        self.shader.set_uniform_1u("u_Idx\0", idx);
        self.shader.unbind();
        self.idx = idx;
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
        self.pos.pos.clone()
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
