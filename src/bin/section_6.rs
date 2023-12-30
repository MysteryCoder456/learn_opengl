// Getting Started - Section 6: Shaders

extern crate gl;
extern crate glfw;

use std::{ffi::CString, os::raw::c_void};

use gl::types::{GLchar, GLint, GLuint};
use glfw::Context;

const INFO_BUFFER_CAPACITY: usize = 512;

// Format: 3 floats -> position and next 3 floats -> color
const TRIANGLE_VERTICES: [f32; 18] = [
    0.0, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
];

struct Shader {
    program: GLuint,
}

impl Shader {
    pub unsafe fn new(
        vertex_shader_path: &str,
        fragment_shader_path: &str,
    ) -> Result<Self, String> {
        let vertex_source = std::fs::read(vertex_shader_path).map_err(|e| e.to_string())?;
        let fragment_source = std::fs::read(fragment_shader_path).map_err(|e| e.to_string())?;

        let vertex_shader_cstr = CString::new(vertex_source).unwrap();
        let fragment_shader_cstr = CString::new(fragment_source).unwrap();

        // Create and compile vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_shader_cstr.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);

        let mut vertex_success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut vertex_success);

        // Check for compilation errors
        let vertex_error = if vertex_success == 0 {
            let mut info_buffer = Vec::with_capacity(INFO_BUFFER_CAPACITY);
            info_buffer.set_len(INFO_BUFFER_CAPACITY - 1);

            gl::GetShaderInfoLog(
                vertex_shader,
                INFO_BUFFER_CAPACITY as i32,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );
            Some(std::str::from_utf8(&info_buffer).unwrap().to_owned())
        } else {
            None
        };

        // Create and compile fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_shader_cstr.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        let mut fragment_success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut fragment_success);

        // Check for compilation errors
        let fragment_error = if fragment_success == 0 {
            let mut info_buffer = Vec::with_capacity(INFO_BUFFER_CAPACITY);
            info_buffer.set_len(INFO_BUFFER_CAPACITY - 1);

            gl::GetShaderInfoLog(
                fragment_shader,
                INFO_BUFFER_CAPACITY as i32,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );
            Some(std::str::from_utf8(&info_buffer).unwrap().to_owned())
        } else {
            None
        };

        // Combine and return both errors if either vertex or fragment shader fails to compile
        if vertex_error.is_some() || fragment_error.is_some() {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            let mut final_error = String::new();
            final_error.push_str(&vertex_error.unwrap_or("".to_owned()));
            final_error.push_str(&fragment_error.unwrap_or("".to_owned()));
            return Err(final_error);
        }

        // Create and link shader program
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // Delete unneeded shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let mut link_success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut link_success);

        // Check for linking errors
        if link_success == 0 {
            let mut info_buffer = Vec::with_capacity(INFO_BUFFER_CAPACITY);
            info_buffer.set_len(INFO_BUFFER_CAPACITY - 1);

            gl::GetProgramInfoLog(
                program,
                INFO_BUFFER_CAPACITY as i32,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );

            gl::DeleteProgram(program);
            Err(std::str::from_utf8(&info_buffer).unwrap().to_owned())
        } else {
            Ok(Self { program })
        }
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.program);
    }

    pub unsafe fn get_uniform_location(&self, uniform_name: &str) -> GLint {
        let name = CString::new(uniform_name).unwrap();
        gl::GetUniformLocation(self.program, name.as_ptr())
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
        let shader = Shader::new(
            "shaders/section_6/vertex.glsl",
            "shaders/section_6/fragment.glsl",
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
