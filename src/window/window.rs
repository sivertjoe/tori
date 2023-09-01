use std::cell::RefCell;

use crate::{core::renderer::Renderer, error::Error, graphics::drawable::Drawable};

type Recv = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;
#[allow(dead_code)]
pub struct Window
{
    glfw:     RefCell<glfw::Glfw>,
    window:   RefCell<glfw::Window>,
    events:   Recv,
    renderer: Renderer,
}

#[allow(dead_code)]
impl Window
{
    pub fn new<S: AsRef<str>>(name: S, width: usize, height: usize) -> Result<Self, Error>
    {
        let mut glfw = glfw::init(glfw::LOG_ERRORS)?;

        let (mut window, events) = glfw
            .create_window(width as _, height as _, name.as_ref(), glfw::WindowMode::Windowed)
            .ok_or(Error::Glfw)?;

        gl::load_with(|s| window.get_proc_address(s));

        glfw.make_context_current(Some(&window));
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        window.set_all_polling(true);

        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        unsafe
        {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
        }

        let renderer = Renderer::new();

        Ok(Self {
            glfw: RefCell::new(glfw),
            window: RefCell::new(window),
            events,
            renderer,
        })
    }

    pub fn poll_events(&self) -> impl Iterator<Item = glfw::WindowEvent> + '_
    {
        self.glfw.borrow_mut().poll_events();
        glfw::flush_messages(&self.events).map(|e| e.1)
    }

    pub fn is_open(&self) -> bool
    {
        !self.window.borrow().should_close()
    }

    pub fn clear(&self)
    {
        self.renderer.clear();
    }

    pub fn draw<D: Drawable>(&self, d: D)
    {
        let va = d.vertex_array();
        let ib = d.index_buffer();
        let shader = d.shader();

        self.renderer.draw(&va, &ib, &shader);
    }
}
