use crate::math;
pub struct Character {
    pub texture_id: u32,
    pub size: math::IVec2,
    pub bearing: math::IVec2,
    pub advance: u32
}
