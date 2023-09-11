use tori::{
    graphics::shape::{Rect, Triangle},
    math::{vec2, vec4},
    window::Window,
};
fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let window = Window::new("Hello", 300, 300)?;

    let triangle = Triangle::new(vec2(150.0, 150.0), vec2(200.0, 200.0), vec2(250.0, 150.0));

    let mut rect = Rect::new(50.0, 50.0, 75.0, 75.0);
    rect.set_color(vec4(0., 0.5, 0.5, 1.0));

    while window.is_open()
    {
        let _ = window.poll_events();

        window.clear();

        window.draw(&triangle);
        window.draw(&rect);

        window.swap_buffers();
    }

    Ok(())
}
