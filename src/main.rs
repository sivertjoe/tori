#[macro_use]
mod renderer;
#[macro_use]
mod util;
mod vertex_buffer;

use glfw::Context;

mod index_buffer;
mod vertex_array;

mod vertex_buffer_layout;

mod shader;


unsafe fn main_()
{
    let mut glfw = glfw::init(glfw::LOG_ERRORS).expect("initing glfw");

    let (mut window, _events) = glfw
        .create_window(300, 300, "hello this is window", glfw::WindowMode::Windowed)
        .unwrap();

    gl::load_with(|s| window.get_proc_address(s));

    glfw.make_context_current(Some(&window));
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));


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

    let mut vao = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    let vb = vertex_buffer::VertexBuffer::new(&positions);
    let mut va = vertex_array::VertexArray::new();
    let mut layout = vertex_buffer_layout::VertexBufferLayout::new();
    layout.push(2, gl::FLOAT);
    va.add_buffer(&vb, layout);

    let ib = index_buffer::IndexBuffer::new(&indices);

    let shader = shader::Shader::new("res/shaders/basic.shader");
    shader.bind();
    shader.set_uniform_f4("u_Color\0", 0.8, 0.3, 0.8, 1.0);

    va.unbind();
    vb.unbind();
    ib.unbind();
    shader.unbind();


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

        shader.bind();
        shader.set_uniform_f4("u_Color\0", r, 0.3, 0.8, 1.0);

        va.bind();
        ib.bind();

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
