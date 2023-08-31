fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let window = tori::window::Window::new("Hello", 300, 300)?;

    while window.is_open()
    {
        let _ = window.poll_events();
        window.clear();
    }

    Ok(())
}
