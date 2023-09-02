use tori::{
    graphics::shape::{Rect, Triangle},
    math::{Point, Vec4},
    window::Window,
};
fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let window = Window::new("Hello", 300, 300)?;

    let triangle = Triangle::new(Point::new(150, 150), Point::new(200, 200), Point::new(250, 150));

    let mut rect = Rect::new(50, 50, 75, 75);
    rect.set_color(Vec4::new(0., 0.5, 0.5, 1.0));

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
