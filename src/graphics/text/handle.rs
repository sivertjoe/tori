use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::graphics::text::{character::Character, quad::Quad};

pub struct Handle(
    pub(crate) usize,
    pub(crate) Rc<Quad>,
    pub(crate) Rc<RefCell<HashMap<(usize, char), Character>>>,
);

impl std::cmp::PartialEq for Handle
{
    fn eq(&self, other: &Handle) -> bool
    {
        self.0 == other.0
    }
}
impl std::cmp::Eq for Handle {}
impl std::hash::Hash for Handle
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    {
        self.0.hash(state);
    }
}
