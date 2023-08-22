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

macro_rules! raw {
    ($p:expr) => {
        $p.as_ptr() as *const std::ffi::c_char
    };
}

macro_rules! gl_call {
    ($fun:expr) => {{
        $fun;
        if !gl_log_call()
        {
            println!("{} yielded an error in {}, line: {}", stringify!($fun), file!(), line!());
            std::process::exit(1);
        }
    }};
}

unsafe fn gl_log_call() -> bool
{
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
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));


    #[rustfmt::skip]
    let positions = [
        -0.5, -0.5, 
         0.5, -0.5, 
         0.5, 0.5,
        -0.5, 0.5_f32,
    ];

    #[rustfmt::skip]
    let indices: [u32; 6] = [
        0, 1, 2, 
        2, 3, 0
    ];

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

    let mut ibo = 0;
    gl::GenBuffers(1, &mut ibo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        size_of_val(&indices) as isize,
        ptr!(indices),
        gl::STATIC_DRAW,
    );

    let (vertex_shader, fragment_shader) = parse_shader("res/shaders/basic.shader");
    let shader = create_shader(vertex_shader.as_bytes(), fragment_shader.as_bytes());
    gl::UseProgram(shader as _);

    let location = gl::GetUniformLocation(shader as _, raw!("u_Color\0"));
    assert!(location != -1);

    let mut r = 0.0;
    let mut inc = 0.05;
    while !window.should_close()
    {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        if r > 1.0
        {
            inc = -0.05;
        }
        else if r < 0.0
        {
            inc = 0.05;
        }
        r += inc;

        gl::Uniform4f(location, r, 0.3, 0.8, 1.0);
        gl_call!(gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null()));

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
