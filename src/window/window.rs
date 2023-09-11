use std::cell::RefCell;

use crate::{
    core::renderer::Renderer,
    error::Error,
    graphics::{
        drawable::Drawable,
        text::{freetype::Freetype, CharSet, Handle},
    },
};

type Recv = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;
pub struct Window
{
    glfw:     RefCell<glfw::Glfw>,
    window:   RefCell<glfw::Window>,
    events:   Recv,
    renderer: Renderer,
    proj:     glm::Mat4,
    freetype: Option<Freetype>,
}

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

        let proj = glm::ortho(0., width as _, 0., height as _, -1., 1.);

        Ok(Self {
            glfw: RefCell::new(glfw),
            window: RefCell::new(window),
            events,
            renderer,
            proj,
            freetype: None,
        })
    }

    pub fn load_font<P>(&mut self, p: P, set: CharSet) -> Result<Handle, Error>
    where
        P: AsRef<std::path::Path>,
    {
        if self.freetype.is_none()
        {
            self.freetype = Some(Freetype::new()?);
        }
        let ft = self.freetype.as_mut().unwrap();
        let h = ft.add_font(p, set)?;

        Ok(h)
    }

    pub fn remove(&mut self)
    {
        self.freetype = None;
    }

    pub fn poll_events(&self) -> impl Iterator<Item = glfw::WindowEvent> + '_
    {
        self.glfw.borrow_mut().poll_events();
        glfw::flush_messages(&self.events).map(|e| e.1)
    }

    pub fn is_key_pressed(&self, key: crate::event::Key) -> bool
    {
        use glfw::Action as A;
        match self.window.borrow().get_key(key)
        {
            A::Press | A::Repeat => true,
            A::Release => false,
        }
    }

    pub fn is_open(&self) -> bool
    {
        !self.window.borrow().should_close()
    }

    pub fn set_open(&self, b: bool)
    {
        self.window.borrow_mut().set_should_close(b);
    }

    pub fn clear(&self)
    {
        self.renderer.clear();
    }

    pub fn swap_buffers(&self)
    {
        use glfw::Context;
        self.window.borrow_mut().swap_buffers();
    }

    pub fn draw<D: Drawable>(&self, d: D)
    {
        d.draw(self.proj);
    }

    pub fn draw_dyn(&self, d: &dyn Drawable)
    {
        d.draw(self.proj);
    }
}

impl Drop for Window
{
    fn drop(&mut self)
    {
        drop(self.freetype.take());
    }
}
