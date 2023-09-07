use crate::math::{self, Mat4, Vec2};

pub struct Entity
{
    pub scale:    Vec2,
    pub pos:      Vec2,
    pub rotation: f32,
}

impl Entity
{
    pub fn new(scale: Vec2, pos: Vec2, rotation: f32) -> Self
    {
        Self {
            scale,
            pos,
            rotation,
        }
    }

    pub fn get_model(&self, size: [f32; 2]) -> Mat4
    {
        let mut model = math::identity();

        model = math::translate(&model, &math::vec3(self.pos[0], self.pos[1], 0.0));

        model = math::translate(&model, &math::vec3(0.5 * size[0], 0.5 * size[1], 0.0));
        model = math::rotate(&model, self.rotation, &glm::vec3(0.0, 0.0, 1.0));
        model = math::translate(&model, &math::vec3(-0.5 * size[0], -0.5 * size[1], 0.0));

        model = math::scale(&model, &math::vec3(self.scale[0], self.scale[1], 1.0));

        model
    }
}
