use std::collections::HashMap;

use freetype::Library;

use crate::{
    core::{shader, util::ptr, vertex_array, vertex_buffer, vertex_buffer_layout},
    error::Error,
    graphics::text::Character,
    math::IVec2,
    util::{get_shader, ShaderProgram::Text},
};

pub struct Quad
{
    pub va:     vertex_array::VertexArray,
    pub vb:     vertex_buffer::VertexBuffer,
    pub shader: shader::Shader,
}

impl Quad
{
    pub fn new() -> Self
    {
        let vb =
            vertex_buffer::VertexBuffer::new_dynamic(6 * 4 * std::mem::size_of::<f32>() as isize);
        let mut va = vertex_array::VertexArray::new();
        let mut layout = vertex_buffer_layout::VertexBufferLayout::new();
        layout.push(4, gl::FLOAT);
        va.add_buffer(&vb, layout);

        let shader = shader::Shader::from_shader_string(get_shader(Text));

        vb.unbind();
        va.unbind();
        shader.unbind();

        Self {
            va,
            vb,
            shader,
        }
    }
}


#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct Handle(usize);

pub struct Freetype
{
    lib:            Library,
    pub characters: HashMap<(Handle, char), Character>,
    pub quad:       Quad,
    idx:            usize,
}

impl Freetype
{
    pub fn new() -> Result<Self, Error>
    {
        unsafe
        {
            // idk if this is needed
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }
        let lib = Library::init()?;
        Ok(Self {
            lib,
            characters: HashMap::new(),
            quad: Quad::new(),
            idx: 0,
        })
    }

    pub fn add_font<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> Result<Handle, freetype::Error>
    {
        let face = self.lib.new_face(path.as_ref(), 0)?;

        face.set_pixel_sizes(0, 48)?;

        let handle = Handle(self.idx);
        self.idx += 1;

        for ch in (0u8 as char)..(128u8 as char)
        {
            face.load_char(ch as usize, freetype::face::LoadFlag::RENDER)?;
            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            let mut texture_id = 0;
            unsafe
            {
                gl::GenTextures(1, &mut texture_id);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as _,
                    bitmap.width(),
                    bitmap.rows(),
                    0,
                    gl::RED as _,
                    gl::UNSIGNED_BYTE,
                    ptr!(bitmap.buffer()),
                );

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            }
            let character = Character {
                texture_id,
                size: IVec2::new(bitmap.width(), bitmap.rows()),
                bearing: IVec2::new(glyph.bitmap_left(), glyph.bitmap_top()),
                advance: glyph.advance().x as _,
            };
            self.characters.insert((handle, ch), character);
        }

        Ok(handle)
    }
}
