use std::time::Instant;

use tori::{
    event::Key,
    graphics::{
        shape::{Circle, Rect},
        text::{Handle, Text},
    },
    math::{vec2, vec4, Vec2, Vec4},
};

pub struct Game
{
    paddle:     Rect,
    score_text: Text,
    score:      isize,
    bricks:     Vec<Rect>,
    info:       Text,
    timer:      Instant,
    toggle:     bool,
    ball:       Circle,
    ball_vel:   Vec2,
}


impl Game
{
    pub fn new(handle: &Handle) -> Self
    {
        let mut bricks = Vec::new();
        let colors: Vec<Vec4> = vec![
            vec4(1.0, 48.0 / 255.0, 54.0 / 255.0, 1.0),
            vec4(245.0 / 255.0, 91.0 / 255.0, 29.0 / 255.0, 1.0),
            vec4(224.0 / 255.0, 175.0 / 255.0, 70.0 / 255.0, 1.0),
            vec4(227.0 / 255.0, 207.0 / 255.0, 170.0 / 255.0, 1.0),
        ];

        // w = 600
        let p = 10.0;
        let w = 89.0;
        let h = 20.0;
        let mut y = 500.0;
        let mut x;

        for color in colors
        {
            x = 0.0;
            for _ in 0..8
            {
                x += p;
                let mut rect = Rect::new(x, y, w, h);
                rect.set_color(color);
                bricks.push(rect);
                x += w;
            }
            y -= h + p;
        }

        let y = 20.0;
        let h = 15.0;
        let x = 250.0;
        let w = 175.0;

        let rect = Rect::new(x, y, w, h);
        let bb = rect.entity.get_bouding_box();

        Self {
            paddle: rect,
            score_text: Text::new(handle, "Score: 0", 20.0, 550.0, 0.4, vec4(1.0, 1.0, 1.0, 1.0)),
            score: 0,
            bricks,
            info: Text::new(
                handle,
                "PRESS SPACE TO LAUNCH",
                200.0,
                300.0,
                0.4,
                vec4(1.0, 1.0, 1.0, 1.0),
            ),
            timer: Instant::now(),
            toggle: true,
            ball: Self::new_ball(bb),
            ball_vel: vec2(0.0, 0.0),
        }
    }

    fn new_ball(paddle: Vec4) -> Circle
    {
        let x = paddle.x;
        let y = paddle.y;
        let w = paddle[2];
        let h = paddle[3];
        let rad = 20.0;
        let cx = x + w / 2.0;
        let cy = ((y + h) + rad / 2.0) + 10.0;

        Circle::new(vec2(cx, cy), rad, vec4(0.7, 0.7, 0.7, 1.0))
    }
}

impl crate::Scene for Game
{
    fn update(&mut self, window: &tori::window::Window) -> Option<crate::NewSceneInfo>
    {
        if window.is_key_pressed(Key::Right)
        {
            self.paddle.entity.pos[0] += 12.0;
        }
        else if window.is_key_pressed(Key::Left)
        {
            self.paddle.entity.pos[0] -= 12.0;
        }

        if self.ball_vel.y == 0.0
        {
            self.ball.center[0] = self.paddle.entity.pos.x + (self.paddle.entity.size.x / 2.0);
            if self.timer.elapsed().as_secs_f32() > 0.55
            {
                self.timer = Instant::now();
                self.toggle = !self.toggle;
                let val = (self.toggle as u32) as f32;
                self.info.color[3] = val;
            }
            if window.is_key_pressed(Key::Space)
            {
                self.ball_vel.y = 5.0;
                self.info.color[3] = 0.0;
            }
        }
        else
        {
            let mut del = None;
            for (i, brick) in self.bricks.iter().enumerate()
            {
                let bb = brick.entity.get_bouding_box();
                if crate::util::circle_rect_intersects(self.ball.center, self.ball.radius, bb)
                {
                    del = Some(i);
                    break;
                }
            }
            if let Some(del) = del
            {
                self.ball_vel.y = -5.0;
                self.bricks.swap_remove(del);
                self.score += 1;
            }

            let bb = self.paddle.entity.get_bouding_box();
            if crate::util::circle_rect_intersects(self.ball.center, self.ball.radius, bb)
            {
                let x = (self.ball.center.x - (bb.x + bb[2] / 2.0)) / 15.0;
                self.ball_vel.x = x;
                self.ball_vel.y = 5.0;
            }

            if self.ball.center.x + self.ball.radius > 800.0
                || self.ball.center.x - self.ball.radius < 0.0
            {
                self.ball_vel.x = -self.ball_vel.x;
            }

            if self.ball.center.y > 600.0
            {
                self.ball_vel.y = -self.ball_vel.y;
            }

            if self.ball.center.y < 0.0
            {
                self.score -= 5;
                let bb = self.paddle.entity.get_bouding_box();
                self.ball = Self::new_ball(bb);
                self.ball_vel = vec2(0.0, 0.0);
            }
        }


        self.ball.center += self.ball_vel;


        self.score_text.text = format!("Score: {}", self.score);

        self.bricks.is_empty().then_some(crate::NewSceneInfo::Game(self.score))
    }

    fn draw(&self, drawer: &dyn Fn(&dyn tori::graphics::drawable::Drawable))
    {
        drawer(&self.paddle);
        drawer(&self.ball);
        drawer(&self.score_text);
        drawer(&self.info);
        for paddle in self.bricks.iter()
        {
            drawer(&paddle);
        }
    }
}
