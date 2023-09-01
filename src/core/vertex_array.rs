use std::ffi::c_void;

use crate::core::{
    util::gl_call, vertex_buffer::VertexBuffer, vertex_buffer_layout::VertexBufferLayout,
};

pub struct VertexArray
{
    renderer_id: u32,
}

impl VertexArray
{
    pub fn new() -> Self
    {
        let mut renderer_id = 0;
        unsafe
        {
            gl::GenVertexArrays(1, &mut renderer_id);
        }

        Self {
            renderer_id,
        }
    }

    pub fn bind(&self)
    {
        unsafe
        {
            gl::BindVertexArray(self.renderer_id);
        }
    }

    pub fn unbind(&self)
    {
        unsafe
        {
            gl::BindVertexArray(0);
        }
    }

    pub fn add_buffer(&mut self, vb: &VertexBuffer, layout: VertexBufferLayout)
    {
        self.bind();
        vb.bind();
        unsafe
        {
            let mut offset = 0;
            for (i, elem) in layout.get_elements().iter().enumerate()
            {
                gl::EnableVertexAttribArray(i as _);
                gl::VertexAttribPointer(
                    i as _,
                    elem.count as _,
                    elem.r#type,
                    elem.normalized,
                    layout.stride as _,
                    offset as *const c_void,
                );
                offset += elem.count * elem.size_of_type();
            }
        }
    }
}

impl Drop for VertexArray
{
    fn drop(&mut self)
    {
        unsafe
        {
            gl_call!(gl::DeleteVertexArrays(1, &self.renderer_id as *const u32));
        }
    }
}
