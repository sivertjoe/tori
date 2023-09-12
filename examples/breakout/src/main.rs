mod game;
mod menu;
mod score;
mod util;

use std::time::Instant;

use tori::{
    graphics::text::{CharSet, Handle, Text},
    math,
};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let mut window = tori::window::Window::new("Hello", 800, 600)?;

    let font = window.load_font("Atari-90.ttf", CharSet::Ascii).unwrap();

    let mut scene: Box<dyn Scene> = Box::new(menu::Menu::new(&font));

    let mut fps = Text::new(&font, "starting..", 500.0, 550.0, 0.4, math::vec4(1.0, 1.0, 1.0, 1.0));
    let mut timer = Instant::now();
    let mut frames = 0.0;

    while window.is_open()
    {
        frames += 1.0;
        if timer.elapsed().as_secs() >= 1
        {
            fps.text = format!("{:.2} ms/frame", 1000.0 / frames);
            timer = Instant::now();
            frames = 0.0;
        }
        let _ = window.poll_events();
        window.clear();

        if let Some(scene_info) = scene.update(&window)
        {
            scene = next_state(scene_info, &font);
        }

        scene.draw(&|d| window.draw_dyn(d));
        window.draw(&fps);

        window.swap_buffers();
    }

    Ok(())
}

fn next_state<'f>(state: NewSceneInfo, handle: &'f Handle) -> Box<dyn Scene + 'f>
{
    match state
    {
        NewSceneInfo::Menu => Box::new(game::Game::new(handle)),
        NewSceneInfo::Game(score) => Box::new(score::Score::new(handle, score)),
    }
}

use tori::graphics::drawable::Drawable;

pub enum NewSceneInfo
{
    Menu,
    Game(isize),
}

pub trait Scene
{
    fn update(&mut self, w: &tori::window::Window) -> Option<NewSceneInfo>;
    fn draw(&self, d: &dyn Fn(&dyn Drawable));
}
