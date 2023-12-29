// Getting Started - Section 6: Shaders

extern crate gl;
extern crate glfw;

use std::{ffi::CString, os::raw::c_void};

use gl::types::{GLchar, GLuint};
use glfw::Context;

const INFO_BUFFER_CAPACITY: usize = 512;
const TRIANGLE_VERTICES: [f32; 9] = [0.0, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0];

const VERTEX_SHADER_SOURCE: &str = r"#version 330 core
layout (location = 0) in vec3 aPos;

void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
";

const FRAGMENT_SHADER_SOURCE: &str = r"#version 330 core
out vec4 FragColor;
uniform vec4 ourColor;

void main() {
    FragColor = ourColor;
}
";

unsafe fn compile_shader(shader: GLuint) -> Result<(), String> {
    // Compile shader
    gl::CompileShader(shader);

    // Check whether compiled successfully
    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

    // Spit back error if did not compile successfully
    if success == 0 {
        let mut info_buffer = Vec::<u8>::with_capacity(INFO_BUFFER_CAPACITY);
        info_buffer.set_len(512 - 1);

        gl::GetShaderInfoLog(
            shader,
            INFO_BUFFER_CAPACITY as i32,
            std::ptr::null_mut(),
            info_buffer.as_mut_ptr() as *mut GLchar,
        );
        Err(std::str::from_utf8(&info_buffer).unwrap().to_owned())
    } else {
        Ok(())
    }
}

unsafe fn link_program(program: GLuint) -> Result<(), String> {
    // Link program
    gl::LinkProgram(program);

    // Check whether linked successfully
    let mut success = 0;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

    // Spit back error if did not link successfully
    if success == 0 {
        let mut info_buffer = Vec::<u8>::with_capacity(INFO_BUFFER_CAPACITY);
        info_buffer.set_len(512 - 1);

        gl::GetProgramInfoLog(
            program,
            INFO_BUFFER_CAPACITY as i32,
            std::ptr::null_mut(),
            info_buffer.as_mut_ptr() as *mut GLchar,
        );
        Err(std::str::from_utf8(&info_buffer).unwrap().to_owned())
    } else {
        Ok(())
    }
}

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
        // Create vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex_shader_source = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_shader_source.as_ptr(),
            std::ptr::null(),
        );

        // Compile vertex shader
        if let Err(e) = compile_shader(vertex_shader) {
            eprintln!("Failed to compile vertex shader:\n{}", e);
        }

        // Create fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_shader_source = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_shader_source.as_ptr(),
            std::ptr::null(),
        );

        // Compile fragment shader
        if let Err(e) = compile_shader(fragment_shader) {
            eprintln!("Failed to compile fragment shader:\n{}", e);
        }

        // Create shader program
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);

        // Link shader program
        if let Err(e) = link_program(shader_program) {
            eprintln!("Failed to link shader program:\n{}", e);
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

    let vertex_color_location = unsafe {
        let uniform_name = CString::new("ourColor").unwrap();
        gl::GetUniformLocation(shader_program, uniform_name.as_ptr())
    };

    while !window.should_close() {
        process_events(&mut window, &events);

        // Rendering commands
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Use our shader program
            gl::UseProgram(shader_program);

            // Set uniform to change triangle color
            let green_value = glfw.get_time().sin().abs();
            gl::Uniform4f(vertex_color_location, 1.0, green_value as f32, 0.2, 1.0);

            // Draw the vertex array
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
