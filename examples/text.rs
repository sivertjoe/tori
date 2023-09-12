use std::time::Instant;

use tori::{
    graphics::text::{CharSet, Text},
    math,
    window::Window,
};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let mut window = Window::new("Hello", 400, 400)?;

    let ascii = window.load_font("OpenSans-Regular.ttf", CharSet::Ascii)?;

    let jap = window.load_font(
        "jap.ttf",
        CharSet::Custom(Box::new("今朝、鳥がきれいに鳴いていました".chars())),
    )?;

    let all = window.load_font("OpenSans-Italic.ttf", CharSet::All)?;

    let mut last_time = Instant::now();
    let mut n_frames = 0;

    let mut fps =
        Text::new(&ascii, "starting...", 0.0, 250.0, 0.75, math::vec4(1.0, 1.0, 0.5, 1.0));

    let jap = Text::new(
        &jap,
        "今朝、鳥がきれいに鳴いていました",
        0.0,
        150.0,
        0.5,
        math::vec4(0.0, 0.75, 0.45, 1.0),
    );

    let text = Text::new(&all, "æøå", 0.0, 0.0, 1.5, math::vec4(0.3, 0.5, 0.1, 1.0));


    while window.is_open()
    {
        n_frames += 1;
        if last_time.elapsed().as_millis() >= 1000
        {
            let t = 1000.0 / (n_frames as f32);
            fps.text = format!("{:.2} ms/frame", t);
            last_time = Instant::now();
            n_frames = 0;
        }


        let _ = window.poll_events();

        window.clear();

        window.draw(&fps);
        window.draw(&jap);
        window.draw(&text);

        window.swap_buffers();
    }


    Ok(())
}
