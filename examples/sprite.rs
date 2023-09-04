use graphics::{sprite::Sprite, texture::Texture};
use tori::{
    graphics::{self},
    window::Window,
};

const LEFT: (isize, u32, u32) = (-1, 0, 7);
const RIGHT: (isize, u32, u32) = (1, 8, 15);

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let window = Window::new("Hello", 300, 300)?;

    let texture = Texture::new("sheet.png")?;
    let mut sprite = Sprite::new(&texture);
    let mut sheet = sprite.make_sprite_sheet(8, 2);

    use std::time::Instant;
    let mut now = Instant::now();
    let mut mov = Instant::now();

    let mut dir = RIGHT;
    sheet.set_idx(8);

    while window.is_open()
    {
        if now.elapsed().as_secs_f32() >= 0.15
        {
            let mut idx = sheet.get_idx() + 1;
            if idx >= dir.2
            {
                idx = dir.1
            }
            sheet.set_idx(idx);
            now = Instant::now();
        }

        if mov.elapsed().as_secs_f32() > 0.03
        {
            mov = Instant::now();
            let mut pos = sprite.pos.get();
            pos[0] += 5_isize.saturating_mul(dir.0);
            sprite.pos.set(pos);
        }

        let v: isize = sprite.pos.get()[0];
        if v < 0 || (v + sprite.get_size()[0] as isize) >= 300
        {
            let nd = if v < 0 { RIGHT } else { LEFT };
            dir = nd;
            now = Instant::now();
            sheet.set_idx(dir.1);
        }

        let _ = window.poll_events();

        window.clear();

        window.draw(&sprite);

        window.swap_buffers();
    }

    Ok(())
}
