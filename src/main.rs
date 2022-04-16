use cgmath::{perspective, SquareMatrix};
use gl::types::{GLfloat, GLsizei, GLsizeiptr};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Mod;

use c_str_macro::c_str;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

mod common;
mod fps_manager;
mod shader;
mod texture_loader;
mod vertex;

use common::print_success_log;
use fps_manager::FPSManager;
use shader::Shader;
use texture_loader::TextureLoader;
use vertex::Vertex;

type Mat4 = cgmath::Matrix4<f32>;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

const FPS_LIMIT: u32 = 60;

const FLOAT_NUM: usize = 8;
const VERTEX_NUM: usize = 36;
const BUFF_SIZE: usize = FLOAT_NUM * VERTEX_NUM;

fn main() {
    // Initialize SDL2
    let sdl_context = match sdl2::init() {
        Ok(sdl) => sdl,
        Err(e) => panic!("Failed to initialize SDL2: {:?}", e),
    };
    print_success_log("Initialize SDL2");

    // Initialize video subsystem
    let video_subsystem = match sdl_context.video() {
        Ok(video_subsystem) => video_subsystem,
        Err(e) => panic!("Failed to initialize video subsystem: {:?}", e),
    };
    print_success_log("Initialize video subsystem");

    // Initialize OpenGL
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 0);
    let (major, minor) = gl_attr.context_version();
    print_success_log(format!("Initialize OpenGL ({}.{})", major, minor).as_str());

    // Create new window
    let window = match video_subsystem
        .window("Main Window", 1280, 720)
        .opengl()
        .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(e) => panic!("Failed to create new window: {:?}", e),
    };
    print_success_log("Create new window");

    // Initialize FPS manager
    let mut fps_manager = FPSManager::new();
    print_success_log("Initialize FPS manager");

    // Initialize texture loader
    let mut texture_loader = TextureLoader::new();
    print_success_log("Initialize texture loader");

    // Initialize OpenGL
    let _gl_context = match window.gl_create_context() {
        Ok(_gl_context) => _gl_context,
        Err(e) => panic!("Failed to initialize OpenGL: {:?}", e),
    };
    print_success_log("Initialize OpenGL");

    // Load shaders
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);
    let shader = Shader::new("src/shader/Basic.vert", "src/shader/Basic.frag");

    // Initialize ImGui
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(Some(match PathBuf::from_str("Config/DefaultGui.ini") {
        Ok(path_buf) => path_buf,
        Err(e) => panic!("Failed to set ini file path: {:?}", e),
    }));
    print_success_log("Initialize ImGui");

    // Initialize imgui-sdl2
    let mut imgui_sdl2_context = imgui_sdl2::ImguiSdl2::new(&mut imgui_context, &window);
    let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_context, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });
    print_success_log("Initialize imgui-sdl2");

    // Load sample texture
    texture_loader.load(Path::new("resource/Texture.png"), "sample_texture");

    // Set drawing buffer
    #[rustfmt::skip]
    let vertices: [f32; BUFF_SIZE] = [
        // x, y, z, normal-x, normal-y, normal-z, tex_coord-x, tex_coord-y
        0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
        1.0, 1.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0,

        0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0,
        1.0, 1.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0,
        1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 1.0,

        0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0,

        0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 1.0,
        1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0,
        1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 1.0,

        0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0,

        0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
        1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0,
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,

        0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,

        0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
        1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
        1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,

        1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
        1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,

        1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0,

        0.0, 1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0,

        0.0, 1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 1.0,
    ];

    // Buffer reference Cube.obj
    /*#[rustfmt::skip]
    let vertices: [f32; BUFF_SIZE] = [
        // x, y, z, normal-x, normal-y, normal-z, tex_coord-x, tex_coord-y
        -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.875, 0.5,
         1.0, 1.0,  1.0, 0.0, 1.0, 0.0, 0.625, 0.75,
         1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.625, 0.5,

         1.0,  1.0, 1.0, 0.0, 0.0, 1.0, 0.625, 0.75,
        -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.375, 1.0,
         1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.375, 0.75,

        -1.0,  1.0,  1.0, -1.0, 0.0, 0.0, 0.625, 0.0,
        -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.375, 0.25,
        -1.0, -1.0,  1.0, -1.0, 0.0, 0.0, 0.375, 0.0,

         1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.375, 0.25,
        -1.0, -1.0,  1.0, 0.0, -1.0, 0.0, 0.375, 0.0,
        -1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.125, 0.5,

        1.0,  1.0, -1.0, 1.0, 0.0, 0.0, 0.625, 0.5,
        1.0, -1.0,  1.0, 1.0, 0.0, 0.0, 0.375, 0.75,
        1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.375, 0.5,

        -1.0,  1.0, -1.0, 0.0, 0.0, -1.0, 0.625, 0.25,
         1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.375, 0.5,
        -1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.375, 0.25,

        -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.875, 0.5,
        -1.0, 1.0,  1.0, 0.0, 1.0, 0.0, 0.875, 0.75,
         1.0, 1.0,  1.0, 0.0, 1.0, 0.0, 0.625, 0.75,

         1.0,  1.0, 1.0, 0.0, 0.0, 1.0, 0.625, 0.75,
        -1.0,  1.0, 1.0, 0.0, 0.0, 1.0, 0.625, 1.0,
        -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.375, 1.0,

        -1.0,  1.0,  1.0, -1.0, 0.0, 0.0, 0.625, 0.0,
        -1.0,  1.0, -1.0, -1.0, 0.0, 0.0, 0.625, 0.25,
        -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.375, 0.25,

         1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.375, 0.5,
         1.0, -1.0,  1.0, 0.0, -1.0, 0.0, 0.375, 0.75,
        -1.0, -1.0,  1.0, 0.0, -1.0, 0.0, 0.125, 0.75,

        1.0,  1.0, -1.0, 1.0, 0.0, 0.0, 0.625, 0.5,
        1.0,  1.0,  1.0, 1.0, 0.0, 0.0, 0.625, 0.75,
        1.0, -1.0,  1.0, 1.0, 0.0, 0.0, 0.375, 0.75,

        -1.0,  1.0, -1.0, 0.0, 0.0, -1.0, 0.625, 0.25,
         1.0,  1.0, -1.0, 0.0, 0.0, -1.0, 0.625, 0.5,
         1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.375, 0.5,
    ];*/

    let vertex = Vertex::new(
        (mem::size_of::<GLfloat>() * BUFF_SIZE) as GLsizeiptr,
        vertices.as_ptr() as *const c_void,
        gl::DYNAMIC_DRAW,
        vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        vec![3, 3, 2],
        mem::size_of::<GLfloat>() as GLsizei * FLOAT_NUM as i32,
        VERTEX_NUM as i32,
    );

    // View settings
    let mut blend = false;
    let mut cull_face = true;
    let mut depth_test = false;
    let mut wire = false;
    let mut camera_x = 2.0f32;
    let mut camera_y = 2.0f32;
    let mut camera_z = 2.0f32;
    let alpha = 1.0f32;
    
    // Material settings
    let mat_shininess = 0.1f32;
    let mat_specular = cgmath::Vector3 {
        x: 0.2f32,
        y: 0.2f32,
        z: 0.2f32,
    };
    
    // Light settings
    let light_ambient = cgmath::Vector3 {
        x: 0.3f32,
        y: 0.3f32,
        z: 0.3f32,
    };
    let light_diffuse = cgmath::Vector3 {
        x: 0.5f32,
        y: 0.5f32,
        z: 0.5f32,
    };
    let light_direction = cgmath::Vector3 {
        x: 1.0f32,
        y: 1.0f32,
        z: 0.0f32,
    };
    let light_specular = cgmath::Vector3 {
        x: 0.2f32,
        y: 0.2f32,
        z: 0.2f32,
    };

    // Main loop until end request (Event processing and Drawing process alternately)
    let mut event_pump = match sdl_context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(e) => panic!("Failed to pump pending event: {:?}", e),
    };

    'main: loop {
        // Execute event process
        for ev in event_pump.poll_iter() {
            // Ignore events on imgui because they are handled on ImGui
            imgui_sdl2_context.handle_event(&mut imgui_context, &ev);
            if imgui_sdl2_context.ignore_event(&ev) {
                continue;
            }

            match ev {
                // Quit event
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,

                // Camera move to X
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    keymod: Mod::LSHIFTMOD,
                    ..
                } => camera_x -= 0.1,
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => camera_x += 0.1,

                // Camera move to Y
                Event::KeyDown {
                    keycode: Some(Keycode::Y),
                    keymod: Mod::LSHIFTMOD,
                    ..
                } => camera_y -= 0.1,
                Event::KeyDown {
                    keycode: Some(Keycode::Y),
                    ..
                } => camera_y += 0.1,

                // Camera move to Z
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    keymod: Mod::LSHIFTMOD,
                    ..
                } => camera_z -= 0.1,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => camera_z += 0.1,

                // Ignore other event
                _ => {}
            }
        }

        // Update view settings
        unsafe {
            if blend {
                gl::Enable(gl::BLEND);
            } else {
                gl::Disable(gl::BLEND);
            }

            if cull_face {
                gl::Enable(gl::CULL_FACE);
            } else {
                gl::Disable(gl::CULL_FACE);
            }

            if depth_test {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }

            if wire {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }

            // Execute drawing process
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // Clear viewport
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Initialize matrices for model, view, projection
            let model_matrix = Mat4::identity();
            let view_matrix = Mat4::look_at_rh(
                cgmath::Point3 {
                    x: camera_x,
                    y: camera_y,
                    z: camera_z,
                },
                cgmath::Point3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5,
                },
                cgmath::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
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
            shader.set_vec(c_str!("uViewPosition"), &cgmath::Vector3::new(camera_x, camera_y, camera_z));
            shader.set_float(c_str!("uAlpha"), alpha);
            shader.set_float(c_str!("uMaterial.shininess"), mat_shininess);
            shader.set_vec(c_str!("uMaterial.specular"), &mat_specular);
            shader.set_vec(c_str!("uLight.ambient"), &light_ambient);
            shader.set_vec(c_str!("uLight.diffuse"), &light_diffuse);
            shader.set_vec(c_str!("uLight.direction"), &light_direction);
            shader.set_vec(c_str!("uLight.specular"), &light_specular);

            // Draw vertices
            gl::BindTexture(gl::TEXTURE_2D, texture_loader.get_from_id("sample_texture"));
            vertex.draw();
            gl::BindTexture(gl::TEXTURE_2D, 0);

            // Draw imgui windows
            imgui_sdl2_context.prepare_frame(
                imgui_context.io_mut(),
                &window,
                &event_pump.mouse_state(),
            );
            let ui = imgui_context.frame();

            // Status UI
            imgui::Window::new("Status")
                .size([250.0, 80.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    let current_fps = fps_manager.get_fps();
                    ui.text(format!("fps: {}", current_fps));

                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({}, {})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });

            // Control panel
            imgui::Window::new("Control Panel")
                .size([300.0, 500.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    ui.checkbox("Blend", &mut blend);
                    ui.checkbox("Cull face", &mut cull_face);
                    ui.checkbox("Depth test", &mut depth_test);
                    ui.checkbox("Wire", &mut wire);
                });

            imgui_sdl2_context.prepare_render(&ui, &window);
            imgui_renderer.render(ui);

            // Update frame
            window.gl_swap_window();
        }

        // FPS limitation
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS_LIMIT));
    }
}
