use std::path::{Path, PathBuf};

use crate::core::util::{gl_call, ptr};

#[allow(dead_code)]
pub struct Texture
{
    renderer_id: u32,
    path:        PathBuf,

    // local_buffer: Box<[u8]>,
    pub width:  i32,
    pub height: i32,
    bpp:        i32,
}

use image::DynamicImage;

fn get_bpp(image: &DynamicImage) -> i32
{
    let color_type = image.color();

    // Determine the bpp based on the color type and number of color channels
    match color_type
    {
        image::ColorType::L8 => 8,
        image::ColorType::La8 => 16,
        image::ColorType::Rgb8 => 24,
        image::ColorType::Rgba8 => 32,
        image::ColorType::L16 => 16,
        image::ColorType::La16 => 32,
        image::ColorType::Rgb16 => 48,
        image::ColorType::Rgba16 => 64,
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
impl Texture
{
    pub fn new<P: AsRef<Path>>(path: P) -> Self
    {
        let mut renderer_id = 0;
        unsafe
        {
            gl::GenTextures(1, &mut renderer_id);
            gl::BindTexture(gl::TEXTURE_2D, renderer_id);

            let mut image = image::open(path.as_ref()).unwrap();
            image::imageops::flip_vertical_in_place(&mut image);

            let width = image.width() as i32;
            let height = image.height() as i32;
            let bpp = get_bpp(&image);

            // let local_buffer = image.as_bytes().to_vec().into_boxed_slice();

            gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _));
            gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _));
            gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _));
            gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _));

            let buffer: *const std::ffi::c_void = ptr!(image.as_bytes());

            gl_call!(gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as _,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                buffer,
            ));
            gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));

            Self {
                renderer_id,
                path: path.as_ref().to_path_buf(),
                // local_buffer,
                width,
                height,
                bpp,
            }
        }
    }

    pub fn bind(&self, slot: Option<u32>)
    {
        unsafe
        {
            let slot = gl::TEXTURE0 + slot.unwrap_or_default();
            gl::ActiveTexture(slot);
            gl::BindTexture(gl::TEXTURE_2D, self.renderer_id);
        }
    }

    pub fn unbind(&self)
    {
        unsafe
        {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture
{
    fn drop(&mut self)
    {
        unsafe
        {
            gl::DeleteTextures(1, &self.renderer_id as *const u32);
        }
    }
}
