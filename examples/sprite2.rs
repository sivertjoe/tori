use graphics::{sprite::Sprite, texture::Texture};
use tori::{
    graphics::{self},
    window::Window,
    event::Key
};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let window = Window::new("Hello", 300, 300)?;

    let texture = Texture::new("sheet.png")?;
    let mut sprite = Sprite::new(&texture);
    let mut sheet = sprite.make_sprite_sheet(8, 2);

    let dur = std::time::Duration::from_secs_f32(0.15);
    sheet.register_event_range(Key::Right, 8, 14, dur.clone());
    sheet.register_event_range(Key::Left, 0, 7, dur.clone());

    while window.is_open()
    {
        let dir = [Key::Left, Key::Right].into_iter().find(|key| window.is_key_pressed(*key));
        sheet.set_direction(dir);

        let speed = match dir
        {
            Some(Key::Left) => -1.0,
            Some(Key::Right) => 1.0,
            _ => 0.0,
        };

        sprite.entity.pos[0] += speed;

        let _ = window.poll_events();

        window.clear();

        window.draw(&sprite);

        window.swap_buffers();
    }

    Ok(())
}
