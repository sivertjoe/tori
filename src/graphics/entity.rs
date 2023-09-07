use crate::math::{self, Mat4, Vec2};

pub struct Entity {
    pub size:     Vec2,
    pub pos:      Vec2,
    pub rotation: f32,
}

impl Entity {
    pub fn new(size: Vec2, pos: Vec2, rotation: f32) -> Self {
        Self {
            size,
            pos,
            rotation
        }
    }

    pub fn get_model(&self) -> Mat4 {
        let mut model = math::identity();

        model = math::translate(&model, &math::vec3(self.pos[0], self.pos[1], 0.0));

        model = math::translate(&model, &math::vec3(0.5 * self.size[0], 0.5 * self.size[1], 0.0));
        model = math::rotate(&model, self.rotation, &glm::vec3(0.0, 0.0, 1.0));
        model = math::translate(&model, &math::vec3(-0.5 * self.size[0], -0.5 * self.size[1], 0.0));

        model = math::scale(&model, &math::vec3(self.size[0], self.size[1], 1.0));

        model
    }
}