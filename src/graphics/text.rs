use crate::{graphics::freetype::Handle, math};

pub struct Text<'handle>
{
    pub text: String,
    handle:   &'handle Handle,
    x:        f32,
    y:        f32,
    scale:    f32,
    color:    math::Vec3,
}

impl<'h> Text<'h>
{
    pub fn new(
        handle: &'h Handle,
        text: impl Into<String>,
        x: f32,
        y: f32,
        scale: f32,
        color: math::Vec3,
    ) -> Self
    {
        Self {
            handle,
            text: text.into(),
            x,
            y,
            color,
            scale,
        }
    }
}

use crate::graphics::drawable::Drawable;
impl<'h> Drawable for Text<'h>
{
    fn draw(&self, proj: math::Mat4)
    {
        let quad = &self.handle.1;
        let shader = &self.handle.1.shader;
        shader.bind();
        shader.set_uniform_f3("u_TextColor\0", self.color.x, self.color.y, self.color.z);
        shader.set_uniform_mat4f("u_Projection\0", &proj);

        let characters = self.handle.2.borrow();

        unsafe
        {
            gl::ActiveTexture(gl::TEXTURE0);
            quad.va.bind();
            let mut x = self.x;

            for c in self.text.chars()
            {
                let ch = characters.get(&(self.handle.0, c)).unwrap();
                let xpos = x + (ch.bearing.x as f32) * self.scale;
                let ypos: f32 = self.y - ((ch.size.y - ch.bearing.y) as f32) * self.scale;

                let w = ch.size.x as f32 * self.scale;
                let h = ch.size.y as f32 * self.scale;

                #[rustfmt::skip]
                let vertices: [f32; 24] = [
                    xpos, ypos + h, 0.0, 0.0,
                    xpos, ypos, 0.0, 1.0,
                    xpos + w, ypos, 1.0, 1.0,

                    xpos, ypos + h, 0.0, 0.0,
                    xpos + w, ypos, 1.0, 1.0,
                    xpos + w, ypos + h, 1.0, 0.0,
                ];

                gl::BindTexture(gl::TEXTURE_2D, ch.texture_id);
                quad.vb.sub_data(&vertices);

                gl::DrawArrays(gl::TRIANGLES, 0, 6);
                x += ((ch.advance >> 6) as f32) * self.scale;
            }

            quad.va.unbind();
            quad.vb.unbind();
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
