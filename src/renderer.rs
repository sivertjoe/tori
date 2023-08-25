#[macro_export]
macro_rules! gl_call {
    ($fun:expr) => {{
        $fun;
        let gl_log_call = || {
            loop
            {
                let error = gl::GetError();
                if error == gl::NO_ERROR
                {
                    break;
                }
                println!("[OpenGL Error] ({:x})", error);
                return false;
            }
            return true;
        };
        if !gl_log_call()
        {
            println!("{} yielded an error in {}, line: {}", stringify!($fun), file!(), line!());
            std::process::exit(1);
        }
    }};
}

use crate::{index_buffer::IndexBuffer, shader::Shader, vertex_array::VertexArray};

pub struct Renderer {}

impl Renderer
{
    pub fn new() -> Self
    {
        Self {}
    }

    pub fn draw(&self, va: &VertexArray, ib: &IndexBuffer, shader: &Shader)
    {
        shader.bind();
        va.bind();
        ib.bind();

        unsafe
        {
            gl_call!(gl::DrawElements(
                gl::TRIANGLES,
                ib.count as _,
                gl::UNSIGNED_INT,
                std::ptr::null()
            ));
        }
    }

    pub fn clear(&self)
    {
        unsafe
        {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
