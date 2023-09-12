use tori::{
    graphics::{
        drawable::Drawable,
        text::{Handle, Text},
    },
    math,
};

pub struct Score<'font>
{
    text: Text<'font>,
}

impl<'f> Score<'f>
{
    pub fn new(handle: &'f Handle, score: isize) -> Self
    {
        let text = Text::new(
            handle,
            format!("TOTAL SCORE: {}", score),
            25.0,
            150.0,
            1.0,
            math::vec4(0.9, 0.9, 0.9, 1.0),
        );

        Self {
            text,
        }
    }
}

impl<'f> crate::Scene for Score<'f>
{
    fn update(&mut self, _window: &tori::window::Window) -> Option<crate::NewSceneInfo>
    {
        None
    }

    fn draw(&self, drawer: &dyn Fn(&dyn Drawable))
    {
        drawer(&self.text);
    }
}
