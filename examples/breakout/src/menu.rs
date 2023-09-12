use math::vec4;
use tori::{
    graphics::{
        drawable::Drawable,
        shape::Rect,
        text::{Handle, Text},
    },
    math,
};

pub struct Menu<'font>
{
    pub welcome_text: Text<'font>,
    pub play_text:    Text<'font>,
    play_rect:        Rect,
}

impl<'f> Menu<'f>
{
    pub fn new(handle: &'f Handle) -> Self
    {
        let play_text =
            Text::new(handle, "PLAY", 300.0, 150.0, 1.0, math::vec4(0.9, 0.9, 0.9, 1.0));
        let bb = play_text.get_bounding_box();
        let rect = tori::graphics::shape::Rect::new(bb.x, bb.y, bb[2], bb[3]);

        Self {
            welcome_text: Text::new(
                handle,
                "BREAKOUT",
                200.0,
                400.0,
                1.0,
                math::vec4(0.9, 0.9, 0.9, 1.0),
            ),
            play_rect: rect,
            play_text,
        }
    }
}

impl<'f> crate::Scene for Menu<'f>
{
    fn update(&mut self, window: &tori::window::Window) -> Option<crate::NewSceneInfo>
    {
        let mp = window.get_mouse_pos();
        let bounding_box = self.play_rect.entity.get_bouding_box();
        if crate::util::point_inside_rect(bounding_box, mp)
        {
            self.play_text.color = vec4(1.0, 1.0, 1.0, 1.0);
        }
        else
        {
            self.play_text.color = vec4(0.9, 0.9, 0.9, 1.0);
        }

        (crate::util::point_inside_rect(bounding_box, mp)
            && window.is_mouse_key_pressed(tori::event::MouseButton::Button1))
        .then_some(crate::NewSceneInfo::Menu)
    }

    fn draw(&self, drawer: &dyn Fn(&dyn Drawable))
    {
        // drawer(&self.play_rect);
        drawer(&self.play_text);
        drawer(&self.welcome_text);
    }
}
