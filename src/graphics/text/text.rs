use crate::{graphics::text::Handle, math};

pub struct Text
{
    pub text:   String,
    pub handle: Handle,
    pub x:      f32,
    pub y:      f32,
    pub scale:  f32,
    pub color:  math::Vec4,
}

impl Text
{
    pub fn new(
        handle: &Handle,
        text: impl Into<String>,
        x: f32,
        y: f32,
        scale: f32,
        color: math::Vec4,
    ) -> Self
    {
        Self {
            handle: handle.clone(),
            text: text.into(),
            x,
            y,
            color,
            scale,
        }
    }

    pub fn get_bounding_box(&self) -> math::Vec4
    {
        let chars = self.handle.1.characters.borrow();

        let mut x = self.x;
        let mut y = None;
        let mut width = 0.0f32;
        let mut height = 0.0f32;

        let len = self.text.len();
        for (i, c) in self.text.chars().enumerate()
        {
            let ch = chars.get(&(self.handle.0, c)).unwrap();
            let ypos = self.y - (ch.size.y as f32 - ch.bearing.y as f32) * self.scale;
            if y.is_none()
            {
                y = Some(ypos);
                x += ch.bearing.x as f32;
            }

            let w = ch.size.x as f32 * self.scale;
            let h = ch.size.y as f32 * self.scale;

            let adv = if i + 1 == len { w } else { (ch.advance >> 6) as f32 * self.scale };

            width += adv;
            height = height.max(h);
        }

        math::vec4(x, y.unwrap_or_default(), width, height)
    }
}

use crate::graphics::drawable::Drawable;
impl Drawable for Text
{
    fn draw(&self, proj: math::Mat4)
    {
        let quad = &self.handle.1.quad;
        let shader = &quad.shader;
        shader.bind();
        shader.set_uniform_f4(
            "u_TextColor\0",
            self.color.x,
            self.color.y,
            self.color.z,
            self.color.w,
        );
        shader.set_uniform_mat4f("u_Projection\0", &proj);

        let characters = self.handle.1.characters.borrow();

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
