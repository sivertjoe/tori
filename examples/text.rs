use std::time::Instant;

use tori::{graphics::text::Text, math, window::Window};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let mut window = Window::new("Hello", 300, 300)?;

    let handle = window.load_font("OpenSans-Regular.ttf")?;

    let mut time = Instant::now();
    let mut n_frames = 0;

    let mut text = Text::new(&handle, "starting...", 0.0, 0.0, 0.75, math::vec3(1.0, 1.0, 0.5));

    while window.is_open()
    {
        n_frames += 1;
        if time.elapsed().as_millis() >= 1000
        {
            let t = 1000.0 / (n_frames as f32);
            text.text = format!("{:.2} ms/frame", t);
            time = Instant::now();
            n_frames = 0;
        }

        let _ = window.poll_events();

        window.clear();

        window.draw(&text);

        window.swap_buffers();
    }


    Ok(())
}
