use std::{cell::RefCell, collections::HashMap};

pub struct Shader
{
    renderer_id: u32,
    _path:       String,

    // I dont want the shader to require being mutable.
    // I'll change it to a mutex if it ever crashes
    // the program
    location_cache: RefCell<HashMap<String, i32>>,
}

#[allow(dead_code)]
impl Shader
{
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Self
    {
        let (vertex_shader, fragment_shader) = Self::parse_shader(path.as_ref());
        let shader = Self::create_shader(vertex_shader.as_bytes(), fragment_shader.as_bytes());

        Self {
            renderer_id:    shader as _,
            _path:          path.as_ref().to_str().unwrap().to_string(),
            location_cache: RefCell::default(),
        }
    }

    pub fn bind(&self)
    {
        unsafe
        {
            gl::UseProgram(self.renderer_id);
        }
    }

    pub fn unbind(&self)
    {
        unsafe
        {
            gl::UseProgram(0);
        }
    }

    pub fn set_uniform_f4(&self, name: &str, v1: f32, v2: f32, v3: f32, v4: f32)
    {
        unsafe
        {
            let location = self.get_uniform_location(name);
            if location == -1
            {
                println!("Warning: location {name} is -1");
            }
            gl::Uniform4f(location, v1, v2, v3, v4);
        }
    }
}

impl Shader
{
    fn get_uniform_location(&self, name: &str) -> i32
    {
        unsafe
        {
            *self
                .location_cache
                .borrow_mut()
                .entry(name.to_string())
                .or_insert_with(|| gl::GetUniformLocation(self.renderer_id, raw!(name)))
        }
    }

    fn create_shader(vertex_shader: &[u8], fragment_shader: &[u8]) -> i32
    {
        unsafe
        {
            let program = gl::CreateProgram();
            let vs = Self::compile_shader(gl::VERTEX_SHADER, vertex_shader);
            let fs = Self::compile_shader(gl::FRAGMENT_SHADER, fragment_shader);

            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
            gl::ValidateProgram(program);

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            program as _
        }
    }

    fn parse_shader<P: AsRef<std::path::Path>>(path: P) -> (String, String)
    {
        let mut vertex = String::new();
        let mut fragment = String::new();
        let mut current = None;

        for line in std::fs::read_to_string(path).unwrap().lines()
        {
            if line.starts_with("#shader")
            {
                if line.contains("vertex")
                {
                    current = Some(&mut vertex);
                }
                else if line.contains("fragment")
                {
                    current = Some(&mut fragment);
                }
                else
                {
                    panic!("Encountered something else..");
                }
                continue;
            }
            if let Some(shader) = current.as_mut()
            {
                shader.push_str(line);
                shader.push('\n');
            }
        }

        vertex.push('\0');
        fragment.push('\0');

        (vertex, fragment)
    }

    fn compile_shader(r#type: u32, src: &[u8]) -> u32
    {
        unsafe
        {
            let id = gl::CreateShader(r#type);

            let ptr = src.as_ptr();
            let ptr_i8: *const i8 = std::mem::transmute(ptr);

            gl::ShaderSource(id, 1, &ptr_i8, std::ptr::null());
            gl::CompileShader(id);


            let mut res = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut res);
            if res as u8 == gl::FALSE
            {
                let mut length = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length);

                let message: Vec<u8> = Vec::with_capacity(length as usize);

                gl::GetShaderInfoLog(
                    id,
                    length,
                    &length as *const i32 as *mut i32,
                    message.as_ptr() as _,
                );

                let s = std::str::from_utf8_unchecked(&message);
                println!(
                    "Failed to compile {} shader {}",
                    if r#type == gl::VERTEX_SHADER { "vertex" } else { "fragment" },
                    s
                );
                gl::DeleteShader(id);
                panic!();
            }

            id
        }
    }
}

impl Drop for Shader
{
    fn drop(&mut self)
    {
        unsafe
        {
            gl::DeleteProgram(self.renderer_id);
        }
    }
}
