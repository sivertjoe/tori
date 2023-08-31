use std::mem::size_of_val;

use crate::core::util::{gl_call, ptr};

pub struct IndexBuffer
{
    renderer_id: u32,
    pub count:   u32,
}

#[allow(dead_code)]
impl IndexBuffer
{
    pub fn new(data: &[u32]) -> Self
    {
        let mut renderer_id = 0;
        unsafe
        {
            gl_call!(gl::GenBuffers(1, &mut renderer_id));
            gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, renderer_id));
            gl_call!(gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                size_of_val(data) as isize,
                ptr!(data),
                gl::STATIC_DRAW,
            ));
        }

        Self {
            renderer_id,
            count: data.len() as _,
        }
    }

    pub fn bind(&self)
    {
        unsafe
        {
            gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.renderer_id));
        }
    }

    pub fn unbind(&self)
    {
        unsafe
        {
            gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));
        }
    }
}


impl Drop for IndexBuffer
{
    fn drop(&mut self)
    {
        let ptr: *const u32 = &self.renderer_id;
        unsafe
        {
            gl_call!(gl::DeleteBuffers(1, ptr));
        }
    }
}
