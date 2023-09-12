use crate::math::{self, Mat4, Vec2, Vec4};

pub struct Entity
{
    pub size:     Vec2,
    pub pos:      Vec2,
    pub rotation: f32,
    pub scale:    Vec2,
}

impl Entity
{
    pub fn new(size: Vec2, pos: Vec2, rotation: f32) -> Self
    {
        Self {
            size,
            pos,
            rotation,
            scale: Vec2::new(1.0, 1.0),
        }
    }

    pub fn get_model(&self) -> Mat4
    {
        let mut model = math::identity();
        let scale = math::vec3(self.size[0] * self.scale[0], self.size[1] * self.scale[1], 1.0);

        model = math::translate(&model, &math::vec3(self.pos[0], self.pos[1], 0.0));

        model = math::translate(&model, &math::vec3(0.5 * scale[0], 0.5 * scale[1], 0.0));
        model = math::rotate(&model, self.rotation, &glm::vec3(0.0, 0.0, 1.0));
        model = math::translate(&model, &math::vec3(-0.5 * scale[0], -0.5 * scale[1], 0.0));

        model = math::scale(&model, &scale);
        model
    }

    pub fn get_bouding_box(&self) -> Vec4
    {
        math::vec4(self.pos.x, self.pos.y, self.size[0], self.size[1])
    }
}
