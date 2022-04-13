use cgmath::{perspective, SquareMatrix};
use gl::types::{GLfloat, GLsizei, GLsizeiptr};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use c_str_macro::c_str;
use std::mem;
use std::os::raw::c_void;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

mod shader;
mod vertex;

use shader::Shader;
use vertex::Vertex;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

const FLOAT_NUM: usize = 3;
const VERTEX_NUM: usize = 3;
const BUFF_SIZE: usize = FLOAT_NUM * VERTEX_NUM;

type Mat4 = cgmath::Matrix4<f32>;

fn main() {
    // Initialize SDL2 system
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Initialize OpenGL
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 0);
    let (major, minor) = gl_attr.context_version();
    println!("OK: Initialize OpenGL ({}.{})", major, minor);

    // Create new window
    let window = video_subsystem
        .window("Main Window", 1280, 720)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    // Create opengl context and shader
    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);
    let shader = Shader::new("src/shader/Basic.vert", "src/shader/Basic.frag");

    // Initialize imgui
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(Some(PathBuf::from_str("Config/DefaultGui.ini").unwrap()));

    // Initialize imgui-sdl2
    let mut imgui_sdl2_context = imgui_sdl2::ImguiSdl2::new(&mut imgui_context, &window);
    let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_context, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });

    // Set drawing buffer
    #[rustfmt::skip]
    let vertices: [f32; BUFF_SIZE] = [
        -1.0, -1.0, 0.0,
         1.0, -1.0, 0.0,
         0.0,  1.0, 0.0,
    ];

    let vertex = Vertex::new(
        (mem::size_of::<GLfloat>() * BUFF_SIZE) as GLsizeiptr,
        vertices.as_ptr() as *const c_void,
        gl::DYNAMIC_DRAW,
        vec![gl::FLOAT],
        vec![FLOAT_NUM as i32],
        mem::size_of::<GLfloat>() as GLsizei * FLOAT_NUM as i32,
        VERTEX_NUM as i32,
    );

    // Main loop until end request (Event processing and Drawing process alternately)
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        // Execute event process
        for ev in event_pump.poll_iter() {
            // Ignore events on imgui because they are handled on ImGui
            imgui_sdl2_context.handle_event(&mut imgui_context, &ev);
            if imgui_sdl2_context.ignore_event(&ev) {
                continue;
            }

            match ev {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        // Execute drawing process
        unsafe {
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // Clear viewport
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Initialize matrices for model, view, projection
            let model_matrix = Mat4::identity();
            let view_matrix = Mat4::look_at_rh(
                cgmath::Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 5.0,
                },
                cgmath::Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                cgmath::Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            );
            let projection_matrix: Mat4 = perspective(
                cgmath::Deg(45.0),
                WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                0.1,
                100.0,
            );

            // Set matrix to shader
            shader.use_program();
            shader.set_mat(c_str!("uModel"), &model_matrix);
            shader.set_mat(c_str!("uView"), &view_matrix);
            shader.set_mat(c_str!("uProjection"), &projection_matrix);

            // Draw vertices
            vertex.draw();

            // Draw imgui window
            imgui_sdl2_context.prepare_frame(
                imgui_context.io_mut(),
                &window,
                &event_pump.mouse_state(),
            );
            let ui = imgui_context.frame();
            imgui::Window::new("ImGui Window")
                .size([300.0, 200.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {});
            imgui_sdl2_context.prepare_render(&ui, &window);
            imgui_renderer.render(ui);

            // Update frame
            window.gl_swap_window();
        }

        // FPS limitation
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
