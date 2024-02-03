// Lighting - Section 12: Colors

extern crate gl;
extern crate glfw;

use std::ffi::c_void;

use glfw::{
    Action, Context, GlfwReceiver, Key, OpenGlProfileHint, PWindow, WindowEvent, WindowHint,
    WindowMode,
};
use learn_opengl::{Camera, Shader};
use nalgebra_glm as glm;

#[rustfmt::skip]
const CUBE_VERTICES: [f32; 180] = [
    //   position    |  texture  |
    -0.5, -0.5, -0.5,  0.0, 0.0,
     0.5, -0.5, -0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0,

     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5,  0.5,  0.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  1.0, 1.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0
];

fn process_events(events: &GlfwReceiver<(f64, WindowEvent)>, window: &mut PWindow) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            WindowEvent::FramebufferSize(w, h) => unsafe {
                gl::Viewport(0, 0, w, h);
            },
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    // Create window
    let (mut window, events) = glfw
        .create_window(800, 600, "Lighting", WindowMode::Windowed)
        .unwrap();
    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s));

    // Enable OpenGL features
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let (light_shader, cube_shader) = unsafe {
        let light_shader = Shader::new(
            "shaders/section_12/cube_vert.glsl",
            "shaders/section_12/light_frag.glsl",
        )
        .unwrap();
        let cube_shader = Shader::new(
            "shaders/section_12/cube_vert.glsl",
            "shaders/section_12/cube_frag.glsl",
        )
        .unwrap();

        (light_shader, cube_shader)
    };

    let cube_vbo = unsafe {
        let mut vbo = 0;

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&CUBE_VERTICES) as isize,
            CUBE_VERTICES.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        vbo
    };

    let light_vao = unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, cube_vbo);
        let f32_size = std::mem::size_of::<f32>() as i32;

        // Position coords
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * f32_size, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindVertexArray(0);
        vao
    };

    let cube_vao = unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, cube_vbo);
        let f32_size = std::mem::size_of::<f32>() as i32;

        // Position coords
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * f32_size, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // Texture coords
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * f32_size,
            (3 * f32_size) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindVertexArray(0);
        vao
    };

    let camera = Camera::new(glm::vec3(0.0, 1.0, 3.0), 45.0, -90.0, -10.0);

    let light_pos = glm::vec3(4.0, 3.0, -6.0);
    let cube_pos = glm::vec3(0.0, 0.0, 0.0);

    while !window.should_close() {
        // Process window events
        process_events(&events, &mut window);

        let mut light_model = glm::Mat4::identity();
        light_model = glm::translate(&light_model, &light_pos);

        let mut cube_model = glm::Mat4::identity();
        cube_model = glm::translate(&cube_model, &cube_pos);

        let view = camera.look_at_matrix();

        let window_size = window.get_size();
        let projection = glm::perspective(
            window_size.0 as f32 / window_size.1 as f32,
            camera.fov(),
            0.1,
            100.0,
        );

        // Light cube uniforms
        unsafe {
            light_shader.use_program();

            gl::UniformMatrix4fv(
                light_shader.get_uniform_location("model"),
                1,
                gl::FALSE,
                glm::value_ptr(&light_model).as_ptr(),
            );
            gl::UniformMatrix4fv(
                light_shader.get_uniform_location("view"),
                1,
                gl::FALSE,
                glm::value_ptr(&view).as_ptr(),
            );
            gl::UniformMatrix4fv(
                light_shader.get_uniform_location("projection"),
                1,
                gl::FALSE,
                glm::value_ptr(&projection).as_ptr(),
            );
        }

        // Normal cube uniforms
        unsafe {
            cube_shader.use_program();

            gl::Uniform3f(
                cube_shader.get_uniform_location("cubeColor"),
                0.972,
                0.514,
                0.475,
            );

            gl::UniformMatrix4fv(
                cube_shader.get_uniform_location("model"),
                1,
                gl::FALSE,
                glm::value_ptr(&cube_model).as_ptr(),
            );
            gl::UniformMatrix4fv(
                cube_shader.get_uniform_location("view"),
                1,
                gl::FALSE,
                glm::value_ptr(&view).as_ptr(),
            );
            gl::UniformMatrix4fv(
                cube_shader.get_uniform_location("projection"),
                1,
                gl::FALSE,
                glm::value_ptr(&projection).as_ptr(),
            );
        }

        // Rendering commands
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Draw light source
            light_shader.use_program();
            gl::BindVertexArray(light_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // Draw cube
            cube_shader.use_program();
            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // Poll events and swap buffers
        window.swap_buffers();
        glfw.poll_events();
    }
}
