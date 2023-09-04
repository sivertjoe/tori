use crate::math;
pub struct Position
{
    pub(crate) pos: math::Mat4,
}

impl Position
{
    pub fn new(pos: math::DVec) -> Self
    {
        let pos =
            glm::translate(&glm::identity::<f32, 4>(), &glm::vec3(pos[0] as _, pos[1] as _, 0.));
        Self {
            pos,
        }
    }

    pub fn get(&self) -> math::DVec
    {
        #[rustfmt::skip]
        /*  0   4    8  12
            1   5    9  13
            2   6   10  14
            3   7   11  15 |*/
        let x = self.pos[12];
        let y = self.pos[13];
        math::DVec::new(x as _, y as _)
    }

    pub fn set(&mut self, pos: math::DVec)
    {
        self.pos[12] = pos[0] as _;
        self.pos[13] = pos[1] as _;
    }
}
