use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::time::Duration;

use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use imgui::im_str;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod image_manager;
mod shader;
mod vertex;

use image_manager::ImageManager;
use shader::Shader;
use vertex::Vertex;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

const WINDOW_WIDTH: u32 = 900;
const WINDOW_HEIGHT: u32 = 480;
const FLOAT_NUM: usize = 8;
const VERTEX_NUM: usize = 36;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);
        let (major, minor) = gl_attr.context_version();
        println!("OK: init OpenGL: version={}.{}", major, minor);
    }

    let window = video_subsystem
        .window("SDL", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    let mut image_manager = ImageManager::new();
    image_manager.load_image(Path::new("rsc/image/surface.png"), "surface", true);

    let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");

    // set buffer
    #[rustfmt::skip]
    let buffer_array: [f32; BUF_LEN] = [
        // 1
        0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
        1.0, 1.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0,

        0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0,
        1.0, 1.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0,
        1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 1.0,

        // 2
        0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0,

        0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 1.0,
        1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0,
        1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 1.0,

        // 3
        0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0,

        0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
        1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0,
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,

        // 4
        0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,

        0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
        1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
        1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,

        // 5
        1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
        1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,

        1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0,

        // 6
        0.0, 1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0,

        0.0, 1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 1.0,
    ];

    let vertex = Vertex::new(
        (BUF_LEN * mem::size_of::<GLfloat>()) as GLsizeiptr,
        buffer_array.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
        vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        vec![3, 3, 2],
        (FLOAT_NUM * mem::size_of::<GLfloat>()) as GLsizei,
        VERTEX_NUM as i32,
    );

    // init imgui
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    // init imgui sdl2
    let mut imgui_sdl2_context = imgui_sdl2::ImguiSdl2::new(&mut imgui_context, &window);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_context, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });

    let mut depth_test: bool = true;
    let mut blend: bool = true;
    let mut wireframe: bool = false;
    let mut culling: bool = true;
    let mut camera_x: f32 = 2.0f32;
    let mut camera_y: f32 = -2.0f32;
    let mut camera_z: f32 = 2.0f32;
    let mut alpha: f32 = 1.0f32;
    let mut material_specular: Vector3 = Vector3 {
        x: 0.2,
        y: 0.2,
        z: 0.2,
    };
    let mut material_shininess: f32 = 0.1f32;
    let mut light_direction: Vector3 = Vector3 {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    };
    let mut ambient: Vector3 = Vector3 {
        x: 0.3,
        y: 0.3,
        z: 0.3,
    };
    let mut diffuse: Vector3 = Vector3 {
        x: 0.5,
        y: 0.5,
        z: 0.5,
    };
    let mut specular: Vector3 = Vector3 {
        x: 0.2,
        y: 0.2,
        z: 0.2,
    };

    let surface_texture_id = image_manager.get_texture_id("surface");

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2_context.handle_event(&mut imgui_context, &event);
            if imgui_sdl2_context.ignore_event(&event) {
                continue;
            }

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        unsafe {
            if depth_test {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }

            if blend {
                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            } else {
                gl::Disable(gl::BLEND);
            }

            if wireframe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }

            if culling {
                gl::Enable(gl::CULL_FACE);
            } else {
                gl::Disable(gl::CULL_FACE);
            }

            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // clear screen
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // init matrice for model, view and projection
            let model_matrix = Matrix4::identity();
            let view_matrix = Matrix4::look_at(
                Point3 {
                    x: camera_x,
                    y: camera_y,
                    z: camera_z,
                },
                Point3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            );
            let projection_matrix: Matrix4 = perspective(
                cgmath::Deg(45.0f32),
                WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                0.1,
                100.0,
            );

            // shader use matrices
            shader.use_program();
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_mat4(c_str!("uView"), &view_matrix);
            shader.set_mat4(c_str!("uProjection"), &projection_matrix);
            shader.set_float(c_str!("uAlpha"), alpha);
            shader.set_vec3(c_str!("uViewPosition"), camera_x, camera_y, camera_z);
            shader.set_vector3(c_str!("uMaterial.specular"), &material_specular);
            shader.set_float(c_str!("uMaterial.shininess"), material_shininess);
            shader.set_vector3(c_str!("uLight.direction"), &light_direction);
            shader.set_vector3(c_str!("uLight.ambient"), &ambient);
            shader.set_vector3(c_str!("uLight.diffuse"), &diffuse);
            shader.set_vector3(c_str!("uLight.specular"), &specular);

            gl::BindTexture(gl::TEXTURE_2D, surface_texture_id as u32);
            vertex.draw();
            gl::BindTexture(gl::TEXTURE_2D, 0);

            imgui_sdl2_context.prepare_frame(
                imgui_context.io_mut(),
                &window,
                &event_pump.mouse_state(),
            );

            let ui = imgui_context.frame();
            imgui::Window::new(im_str!("Information"))
                .size([300.0, 300.0], imgui::Condition::FirstUseEver)
                .position([10.0, 10.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("OpenGL Test App ver 1.0"));
                    ui.separator();
                    ui.text(im_str!("FPS: {:.1}", ui.io().framerate));
                    let display_size = ui.io().display_size;
                    ui.text(format!(
                        "Display Size: ({:.1}, {:.1})",
                        display_size[0], display_size[1]
                    ));
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1}, {:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));

                    ui.separator();

                    ui.checkbox(im_str!("Depth Test"), &mut depth_test);
                    ui.checkbox(im_str!("Blend"), &mut blend);
                    ui.checkbox(im_str!("Wireframe"), &mut wireframe);
                    ui.checkbox(im_str!("Culling"), &mut culling);

                    ui.separator();

                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Camera X"), -5.0..=5.0)
                        .build(&ui, &mut camera_x);
                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Camera Y"), -5.0..=5.0)
                        .build(&ui, &mut camera_y);
                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Camera Z"), -5.0..=5.0)
                        .build(&ui, &mut camera_z);
                });

            imgui::Window::new(im_str!("Light"))
                .size([300.0, 450.0], imgui::Condition::FirstUseEver)
                .position([600.0, 10.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    imgui::Slider::new(im_str!("Alpha"), 0.0..=1.0).build(&ui, &mut alpha);

                    ui.separator();

                    imgui::Slider::new(im_str!("Material Specular X"), 0.0..=1.0)
                        .build(&ui, &mut material_specular.x);
                    imgui::Slider::new(im_str!("Material Specular Y"), 0.0..=1.0)
                        .build(&ui, &mut material_specular.y);
                    imgui::Slider::new(im_str!("Material Specular Z"), 0.0..=1.0)
                        .build(&ui, &mut material_specular.z);

                    imgui::Slider::new(im_str!("Material Shininess"), 0.0..=2.0)
                        .build(&ui, &mut material_shininess);

                    ui.separator();

                    imgui::Slider::new(im_str!("Direction X"), -1.0..=1.0)
                        .build(&ui, &mut light_direction.x);
                    imgui::Slider::new(im_str!("Direction Y"), -1.0..=1.0)
                        .build(&ui, &mut light_direction.y);
                    imgui::Slider::new(im_str!("Direction Z"), -1.0..=1.0)
                        .build(&ui, &mut light_direction.z);

                    ui.separator();

                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Ambient R"), 0.0..=1.0)
                        .build(&ui, &mut ambient.x);
                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Ambient G"), 0.0..=1.0)
                        .build(&ui, &mut ambient.y);
                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Ambient B"), 0.0..=1.0)
                        .build(&ui, &mut ambient.z);

                    ui.separator();

                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Diffuse R"), 0.0..=1.0)
                        .build(&ui, &mut diffuse.x);
                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Diffuse G"), 0.0..=1.0)
                        .build(&ui, &mut diffuse.y);
                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Diffuse B"), 0.0..=1.0)
                        .build(&ui, &mut diffuse.z);

                    ui.separator();

                    imgui::Slider::new(im_str!("Specular R"), 0.0..=1.0)
                        .build(&ui, &mut specular.x);
                    imgui::Slider::new(im_str!("Specular G"), 0.0..=1.0)
                        .build(&ui, &mut specular.y);
                    imgui::Slider::new(im_str!("Specular B"), 0.0..=1.0)
                        .build(&ui, &mut specular.z);
                });

            imgui_sdl2_context.prepare_render(&ui, &window);
            renderer.render(ui);

            window.gl_swap_window();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
