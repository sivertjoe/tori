use crate::math::{self, Mat4, Vec2};

pub struct Entity {
    pub(crate) size: Vec2,
    pub(crate) pos: Vec2,
    pub(crate) rotation: f32,
}

impl Entity {
    pub fn new(size: Vec2, pos: Vec2, rotation: f32) -> Self {
        Self {
            size,
            pos,
            rotation
        }
    }

    pub fn get_size(&self) -> &Vec2 {
        &self.size
    }
    pub fn set_size(&mut self, new_size: Vec2) {
        self.size = new_size;
    }

    pub fn get_pos(&self) -> &Vec2 {
        &self.pos
    }
    pub fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
    pub fn rotate(&mut self, rotation_inc: f32) {
        self.rotation += rotation_inc;
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
