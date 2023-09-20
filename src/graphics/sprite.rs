use std::rc::Rc;

use crate::{
    core::*,
    graphics::{entity::Entity, texture},
    math,
    util::{get_shader, ShaderProgram::Texture},
};

pub struct Sprite<'texture>
{
    va:     vertex_array::VertexArray,
    vb:     vertex_buffer::VertexBuffer,
    ib:     index_buffer::IndexBuffer,
    shader: Rc<shader::Shader>,

    pub entity: Entity,
    texture:    &'texture texture::Texture,
}

impl<'tex> Sprite<'tex>
{
    pub fn new(texture: &'tex texture::Texture) -> Self
    {
        let core = texture.get_core();
        let w = core.width as f32;
        let h = core.height as f32;
        #[rustfmt::skip]
        let positions: [f32; 16] = [
             0.0, 0.0,  0., 0., // bottom left
             1.0, 0.0,  1., 0., // bottom right
             1.0, 1.0,  1., 1., // top right
             0.0, 1.0,  0., 1.  // top left
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
            entity: Entity::new(math::vec2(w, h), math::vec2(0.0, 0.0), 0.0),
        }
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

        self.entity.size[0] /= num_cols as f32;
        self.entity.size[1] /= num_rows as f32;

        SpriteSheet {
            shader: Rc::clone(&self.shader),
            num_cols,
            num_rows,
            idx: 0,
            frames: Vec::new(),
            current_dir: None,
        }
    }
}

struct FrameIndexRange 
{
    key:    crate::event::Key, 
    start:  u32,
    end:    u32,
    duration: Duration,
}

impl FrameIndexRange
{
    fn contains(&self, val: u32) -> bool
    {
        (self.start..=self.end).contains(&val)
    }
}


use std::time::{Instant, Duration};
pub struct SpriteSheet
{
    shader:   Rc<shader::Shader>,
    num_cols: u32,
    num_rows: u32,
    pub idx:  u32,

    frames: Vec<FrameIndexRange>,
    current_dir: Option<(crate::event::Key, Instant)>,
    
}

impl SpriteSheet
{
    pub fn register_event_range(&mut self, key: crate::event::Key, start: u32, end: u32, duration: Duration)
    {
        self.frames.push(
            FrameIndexRange {
                key,
                start,
                end,
                duration
            });
    }
    pub fn set_direction(&mut self, key: Option<crate::event::Key>)
    {
        self.current_dir = key.map(|s| (s, Instant::now()));
    }
    
    pub fn update_index(&mut self) -> bool
    {
        let Some((key, timer)) = self.current_dir.as_mut() else { return false; };

        if let Some(index_range) = self.frames.iter().find(|r| r.key == *key)
        {
            // I always want to set the direction to the default direction.
            if !index_range.contains(self.idx)
            {
                self.idx = index_range.start
            }
            // However, I only want to update `idx` after the duration
            else if timer.elapsed() >= index_range.duration
            {
                *timer = Instant::now();
                self.idx = (self.idx + 1) % (index_range.end + 1);
            };

            true
        }
        else
        {
            false
        }
    }

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

use crate::graphics::drawable::{std_draw, Drawable};
impl<'t> Drawable for Sprite<'t>
{
    fn draw(&self, proj: math::Mat4)
    {
        let model = self.entity.get_model();
        std_draw(&self.va, &self.ib, &self.shader, model, proj, Some(&self.texture.texture));
    }
}
