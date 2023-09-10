use std::rc::Rc;

use crate::graphics::text::freetype::Inner;

#[derive(Clone)]
pub struct Handle(pub(crate) usize, pub(crate) Rc<Inner>);
