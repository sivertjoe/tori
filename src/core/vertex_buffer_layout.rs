pub struct VertexBufferElement
{
    pub count:      u32,
    pub r#type:     u32,
    pub normalized: u8,
}

#[allow(dead_code)]
impl VertexBufferElement
{
    pub fn size_of_type(&self) -> u32
    {
        use std::mem::size_of;

        use gl::types::*;
        (match self.r#type
        {
            gl::FLOAT => size_of::<GLfloat>(),
            gl::UNSIGNED_INT => size_of::<GLuint>(),
            gl::UNSIGNED_BYTE => size_of::<GLubyte>(),
            _ => panic!("Unsupported type"),
        }) as u32
    }
}

pub struct VertexBufferLayout
{
    elements:   Vec<VertexBufferElement>,
    pub stride: i32,
}


#[allow(dead_code)]
impl VertexBufferLayout
{
    pub fn new() -> Self
    {
        Self {
            elements: Vec::new(), stride: 0
        }
    }

    pub fn push(&mut self, count: u32, r#type: u32)
    {
        let normalized = if r#type == gl::UNSIGNED_BYTE { gl::TRUE } else { gl::FALSE };
        let el = VertexBufferElement {
            count,
            r#type,
            normalized,
        };
        self.stride += (el.size_of_type() * count) as i32;
        self.elements.push(el);
    }

    pub fn get_elements(&self) -> &[VertexBufferElement]
    {
        &self.elements
    }
}
