use std::path::Path;

use crate::{core, error::Error};

pub struct Texture
{
    pub(crate) texture: core::texture::Texture,
}

impl Texture
{
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error>
    {
        let texture = core::texture::Texture::new(path)?;
        // texture.bind(None);
        // shader.set_uniform_1i("u_Texture\0", 0);

        Ok(Self {
            texture,
        })
    }

    pub(crate) fn get_core(&self) -> &core::texture::Texture
    {
        &self.texture
    }
}
