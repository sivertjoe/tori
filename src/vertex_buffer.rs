use std::mem::size_of;


pub struct VertexBuffer
{
    renderer_id: u32,
}

impl VertexBuffer
{
    pub fn new<U>(data: &[U]) -> Self
    {
        let mut renderer_id = 0;
        unsafe
        {
            gl_call!(gl::GenBuffers(1, &mut renderer_id));
            gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, renderer_id));
            gl_call!(gl::BufferData(
                gl::ARRAY_BUFFER,
                (size_of::<U>() * data.len()) as isize,
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
