use std::{
    ffi::c_void,
    mem::{size_of, size_of_val},
};

use glfw::Context;

macro_rules! ptr {
    ($p:expr) => {
        $p.as_ptr() as *const std::ffi::c_void
    };
}

unsafe fn compile_shader(r#type: u32, src: &[u8]) -> u32
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

        gl::GetShaderInfoLog(id, length, &length as *const i32 as *mut i32, message.as_ptr() as _);

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

unsafe fn create_shader(vertex_shader: &[u8], fragment_shader: &[u8]) -> i32
{
    let program = gl::CreateProgram();
    let vs = compile_shader(gl::VERTEX_SHADER, vertex_shader);
    let fs = compile_shader(gl::FRAGMENT_SHADER, fragment_shader);

    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    gl::ValidateProgram(program);

    gl::DeleteShader(vs);
    gl::DeleteShader(fs);

    program as _
}

unsafe fn main_()
{
    let mut glfw = glfw::init(glfw::LOG_ERRORS).expect("initing glfw");

    let (mut window, _events) = glfw
        .create_window(300, 300, "hello this is window", glfw::WindowMode::Windowed)
        .unwrap();

    gl::load_with(|s| window.get_proc_address(s));

    glfw.make_context_current(Some(&window));


    let positions: [f32; 6] = [-0.5, -0.5, 0.0, 0.5, 0.5, -0.5];

    let mut buffer = 0;
    gl::GenBuffers(1, &mut buffer);
    gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        size_of_val(&positions) as isize,
        ptr!(positions),
        gl::STATIC_DRAW,
    );

    gl::EnableVertexAttribArray(0);

    gl::VertexAttribPointer(
        0,
        2,
        gl::FLOAT,
        gl::FALSE,
        2 * size_of::<f32>() as i32,
        0 as *const c_void,
    );


    let vertex_shader = "
        #version 330 core
        layout(location = 0) in vec4 position;
        void main()
        {
            gl_Position = position;
        }\0";

    let fragment_shader = "
        #version 330 core
        layout(location = 0) out vec4 color;
        void main()
        {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }\0";
    let shader = create_shader(vertex_shader.as_bytes(), fragment_shader.as_bytes());
    gl::UseProgram(shader as _);


    while !window.should_close()
    {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        window.swap_buffers();

        glfw.poll_events();
    }
}

fn main()
{
    unsafe
    {
        main_();
    }
}
