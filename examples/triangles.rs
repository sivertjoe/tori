use tori::{
    graphics::shape::Triangle,
    math::{Point, Vec4},
    window::Window,
};
fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let window = Window::new("Hello", 300, 300)?;

    let triangle = Triangle::new(Point::new(150, 150), Point::new(200, 200), Point::new(250, 150));

    let mut triangle2 = Triangle::new(Point::new(0, 0), Point::new(50, 50), Point::new(0, 50));
    triangle2.set_color(Vec4::new(0.0, 1.0, 0.0, 1.0));

    while window.is_open()
    {
        let _ = window.poll_events();

        window.clear();

        window.draw(&triangle);
        window.draw(&triangle2);

        window.swap_buffers();
    }

    Ok(())
}
