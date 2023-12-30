// Getting Started - Section 6: Shaders - Exercise 1

extern crate gl;
extern crate glfw;

use std::os::raw::c_void;

use glfw::Context;
use learn_opengl::Shader;

// Format: 3 floats -> position and next 3 floats -> color
const TRIANGLE_VERTICES: [f32; 18] = [
    0.0, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
];

fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(w, h) => unsafe {
                gl::Viewport(0, 0, w, h);
            },
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    // Configure GLFW
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Create a window using GLFW
    let (mut window, events) = glfw
        .create_window(800, 600, "Getting Started", glfw::WindowMode::Windowed)
        .expect("GLFW sh*t itself :(");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|sym| window.get_proc_address(sym) as *const _);

    let shader_program = unsafe {
        let shader = Shader::new(
            "shaders/section_6_exercise_1/vertex.glsl",
            "shaders/section_6_exercise_1/fragment.glsl",
        );

        if let Err(e) = shader {
            panic!("Failed to load shaders:\n{}", e);
        }
        shader.unwrap()
    };

    let vao = unsafe {
        // Create a buffer and vertex array object
        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        // Bind the vertex array first
        gl::BindVertexArray(vao);

        // Bind the buffer object as a vertex buffer and add data to it
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&TRIANGLE_VERTICES) as isize,
            &TRIANGLE_VERTICES as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        let stride = 6 * std::mem::size_of::<f32>() as i32;

        // Configure vertex position attributes
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // Configure vertex color attributes
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        // Unbind vertex array
        gl::BindVertexArray(0);

        vao
    };

    //let vertex_color_location = unsafe {
    //    shader_program.get_uniform_location("vertexColor")
    //};

    while !window.should_close() {
        process_events(&mut window, &events);

        // Rendering commands
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Use our shader program
            shader_program.use_program();

            // Set uniform to change triangle color
            //let green_value = glfw.get_time().sin().abs();
            //gl::Uniform4f(vertex_color_location, 1.0, green_value as f32, 0.2, 1.0);

            // Draw the vertex array
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
