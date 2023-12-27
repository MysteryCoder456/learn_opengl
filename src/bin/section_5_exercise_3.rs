// Getting Started - Section 5: Hello Triangle - Exercise 3

extern crate gl;
extern crate glfw;

use std::{ffi::CString, os::raw::c_void};

use gl::types::GLchar;
use glfw::Context;

const TRIANGLE_VERTICES: [f32; 18] = [
    -0.25, 0.25, 0.0, -0.5, -0.25, 0.0, 0.0, -0.25, 0.0, 0.25, 0.25, 0.0, 0.5, -0.25, 0.0, 0.0,
    -0.25, 0.0,
];

const VERTEX_SHADER_SOURCE: &str = r"#version 330 core
layout (location = 0) in vec3 aPos;

void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
";

const ORANGE_FRAGMENT_SHADER_SOURCE: &str = r"#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0, 0.5, 0.2, 1.0);
}
";

const YELLOW_FRAGMENT_SHADER_SOURCE: &str = r"#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0, 1.0, 0.2, 1.0);
}
";

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

    let orange_shader_program = unsafe {
        // NOTE: VERTEX SHADER

        // Create vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);

        // Add vertex shader source code to the vertex shader
        let vertex_shader_source = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_shader_source.as_ptr(),
            std::ptr::null(),
        );

        // Compile vertex shader
        gl::CompileShader(vertex_shader);

        // Check if vertex shader compiled successfully
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        // If compilation failed, get shader info log and print it
        if success == 0 {
            let mut info_buffer = Vec::<u8>::with_capacity(512);
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "Failed to compile vertex shader:\n{}",
                std::str::from_utf8(&info_buffer).unwrap()
            );
        }

        // NOTE: FRAGMENT SHADER

        // Create fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

        // Add fragment shader source to the fragment shader
        let fragment_shader_source =
            CString::new(ORANGE_FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_shader_source.as_ptr(),
            std::ptr::null(),
        );

        // Compile fragment shader
        gl::CompileShader(fragment_shader);

        // Check if fragment shader compiled successfully
        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);

        // If compilation failed, get shader info log and print it
        if success == 0 {
            let mut info_buffer = Vec::<u8>::with_capacity(512);
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "Failed to compile fragment shader:\n{}",
                std::str::from_utf8(&info_buffer).unwrap()
            );
        }

        // NOTE: SHADER PROGRAM

        // Create shader program
        let shader_program = gl::CreateProgram();

        // Attach shaders to program and link
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check if program linked successfully
        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        // If linking failed, get program info log and print it
        if success == 0 {
            let mut info_buffer = Vec::<u8>::with_capacity(512);
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "Failed to link shader program:\n{}",
                std::str::from_utf8(&info_buffer).unwrap()
            );
        }

        // Delete individual shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    };

    let yellow_shader_program = unsafe {
        // NOTE: VERTEX SHADER

        // Create vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);

        // Add vertex shader source code to the vertex shader
        let vertex_shader_source = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_shader_source.as_ptr(),
            std::ptr::null(),
        );

        // Compile vertex shader
        gl::CompileShader(vertex_shader);

        // Check if vertex shader compiled successfully
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        // If compilation failed, get shader info log and print it
        if success == 0 {
            let mut info_buffer = Vec::<u8>::with_capacity(512);
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "Failed to compile vertex shader:\n{}",
                std::str::from_utf8(&info_buffer).unwrap()
            );
        }

        // NOTE: FRAGMENT SHADER

        // Create fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

        // Add fragment shader source to the fragment shader
        let fragment_shader_source =
            CString::new(YELLOW_FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_shader_source.as_ptr(),
            std::ptr::null(),
        );

        // Compile fragment shader
        gl::CompileShader(fragment_shader);

        // Check if fragment shader compiled successfully
        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);

        // If compilation failed, get shader info log and print it
        if success == 0 {
            let mut info_buffer = Vec::<u8>::with_capacity(512);
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "Failed to compile fragment shader:\n{}",
                std::str::from_utf8(&info_buffer).unwrap()
            );
        }

        // NOTE: SHADER PROGRAM

        // Create shader program
        let shader_program = gl::CreateProgram();

        // Attach shaders to program and link
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check if program linked successfully
        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        // If linking failed, get program info log and print it
        if success == 0 {
            let mut info_buffer = Vec::<u8>::with_capacity(512);
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "Failed to link shader program:\n{}",
                std::str::from_utf8(&info_buffer).unwrap()
            );
        }

        // Delete individual shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
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

        // Configure & enable vertex attributes
        gl::VertexAttribPointer(
            0,                                     // layout location value specified in vertex shader
            3,                                     // Number of dimensions of input vertex
            gl::FLOAT,                             // Data type of the vertex
            gl::FALSE,                             // Normalized?
            3 * std::mem::size_of::<f32>() as i32, // Stride length
            std::ptr::null(),                      // Offset value
        );
        gl::EnableVertexAttribArray(0);

        // Unbind vertex buffer, then vertex array
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        vao
    };

    while !window.should_close() {
        process_events(&mut window, &events);

        // Rendering commands
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);

            gl::UseProgram(orange_shader_program);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::UseProgram(yellow_shader_program);
            gl::DrawArrays(gl::TRIANGLES, 3, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
