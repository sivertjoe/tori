#[macro_use]
mod renderer;
#[macro_use]
mod util;
mod vertex_buffer;

use imgui_glfw_rs::{glfw, glfw::Context, imgui::im_str};

mod index_buffer;
mod vertex_array;

mod vertex_buffer_layout;

mod shader;
mod texture;

const W: u32 = 1024;
const H: u32 = 768;

unsafe fn main_()
{
    let mut glfw = glfw::init(glfw::LOG_ERRORS).expect("initing glfw");

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, _events) = glfw
        .create_window(W, H, "hello this is window", glfw::WindowMode::Windowed)
        .unwrap();

    gl::load_with(|s| window.get_proc_address(s));

    window.make_current();
    window.set_all_polling(true);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));


    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    gl::Enable(gl::BLEND);


    let mut imgui = imgui_glfw_rs::imgui::Context::create();
    imgui.set_ini_filename(None);
    let mut imgui_glfw = imgui_glfw_rs::ImguiGLFW::new(&mut imgui, &mut window);



    #[rustfmt::skip]
    let positions = [
         -50.0_f32,  -50., 0., 0.,  // bottom left
         50., -50., 1., 0.,  // bottom right
         50., 50., 1.0, 1.0, // top right
         -50., 50., 0., 1.// top left
    ];

    #[rustfmt::skip]
    let indices: [u32; 6] = [
        0, 1, 2, 
        2, 3, 0
    ];

    let vb = vertex_buffer::VertexBuffer::new(&positions);
    let mut va = vertex_array::VertexArray::new();
    let mut layout = vertex_buffer_layout::VertexBufferLayout::new();
    layout.push(2, gl::FLOAT);
    layout.push(2, gl::FLOAT);
    va.add_buffer(&vb, layout);


    let proj = glm::ortho(0., W as f32, 0., H as f32, -1., 1.);

    let ib = index_buffer::IndexBuffer::new(&indices);

    let shader = shader::Shader::new("res/shaders/basic.shader");
    shader.bind();
    shader.set_uniform_f4("u_Color\0", 0.8, 0.3, 0.8, 1.0);

    let texture = texture::Texture::new("res/textures/bird.png");
    texture.bind(None);
    shader.set_uniform_1i("u_Texture\0", 0);

    va.unbind();
    vb.unbind();
    ib.unbind();
    shader.unbind();

    let renderer = renderer::Renderer::new();

    let mut translation = glm::vec3(200., 200., 0.);
    let mut translation2 = glm::vec3(100., 300., 0.);
    let ident = glm::identity::<f32, 4>();
    let view = glm::translate(&ident, &glm::vec3(-0., 0., 0.));
    while !window.should_close()
    {
        renderer.clear();

        let ident = glm::identity::<f32, 4>();

        shader.bind();

        {
            let model = glm::translate(&ident, &translation);
            let mvp = proj * view * model;
            shader.set_uniform_mat4f("u_MVP\0", &mvp);
            renderer.draw(&va, &ib, &shader);
        }
        {
            let model = glm::translate(&ident, &translation2);
            let mvp = proj * view * model;
            shader.set_uniform_mat4f("u_MVP\0", &mvp);
            renderer.draw(&va, &ib, &shader);
        }

        let ui = imgui_glfw.frame(&mut window, &mut imgui);

        ui.slider_float(im_str!("width1"), translation.get_mut(0).unwrap(), 0., W as _)
            .build();

        ui.slider_float(im_str!("height1"), translation.get_mut(1).unwrap(), 0., H as _)
            .build();
        ui.slider_float(im_str!("width"), translation2.get_mut(0).unwrap(), 0., W as _)
            .build();

        ui.slider_float(im_str!("height"), translation2.get_mut(1).unwrap(), 0., H as _)
            .build();

        imgui_glfw.draw(ui, &mut window);

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&_events)
        {
            imgui_glfw.handle_event(&mut imgui, &event);
        }
    }
}

fn main()
{
    unsafe
    {
        main_();
    }
}
