use crate::core::util::{gl_call, ptr};


pub struct VertexBuffer
{
    renderer_id: u32,
}

impl VertexBuffer
{
    pub fn new_dynamic(size: isize) -> Self {
        let mut renderer_id = 0;
        unsafe {
            gl_call!(gl::GenBuffers(1, &mut renderer_id));
            gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, renderer_id));
            gl_call!(gl::BufferData(
                gl::ARRAY_BUFFER,
                size,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            ));
        }

        Self {
            renderer_id,
        }
    }

    pub fn sub_data<U>(&self, data: &[U]) {
        use std::mem::size_of_val;
        self.bind();
        unsafe {
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, size_of_val(data) as isize, ptr!(data));
        }
        self.unbind();

    }

    pub fn new<U>(data: &[U]) -> Self
    {
        let mut renderer_id = 0;
        unsafe
        {
            gl_call!(gl::GenBuffers(1, &mut renderer_id));
            gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, renderer_id));
            gl_call!(gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(data) as isize,
                ptr!(data),
                gl::STATIC_DRAW,
            ));
        }

        Self {
            renderer_id,
        }
    }

    pub fn bind(&self)
    {
        unsafe
        {
            gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, self.renderer_id));
        }
    }

    pub fn unbind(&self)
    {
        unsafe
        {
            gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, 0));
        }
    }
}


impl Drop for VertexBuffer
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
