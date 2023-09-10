use std::{cell::RefCell, collections::HashMap, rc::Rc};

use freetype::Library;

use crate::{
    core::util::ptr,
    error::Error,
    graphics::text::{character::Character, quad::Quad, CharSet, Handle},
    math::IVec2,
};

pub struct Freetype
{
    lib:            Library,
    pub characters: Rc<RefCell<HashMap<(usize, char), Character>>>,
    pub quad:       Rc<Quad>,
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
            characters: Rc::default(),
            quad: Rc::new(Quad::new()),
            idx: 0,
        })
    }

    fn add_char(
        &mut self,
        face: &freetype::Face,
        ch: char,
        handle_key: usize,
    ) -> Result<(), freetype::Error>
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
        self.characters.borrow_mut().insert((handle_key, ch), character);
        Ok(())
    }

    pub fn add_font<P>(&mut self, path: P, set: CharSet) -> Result<Handle, freetype::Error>
    where
        P: AsRef<std::path::Path>,
    {
        let face = self.lib.new_face(path.as_ref(), 0)?;

        face.set_pixel_sizes(0, 48)?;

        let handle = Handle(self.idx, Rc::clone(&self.quad), Rc::clone(&self.characters));
        self.idx += 1;

        match set
        {
            CharSet::Ascii =>
            {
                for ch in (0u8 as char)..=(255u8 as char)
                {
                    self.add_char(&face, ch, handle.0)?;
                }
            },
            CharSet::Custom(iter) =>
            {
                for ch in iter.into_iter()
                {
                    self.add_char(&face, ch, handle.0)?;
                }
            },
        }

        Ok(handle)
    }
}
