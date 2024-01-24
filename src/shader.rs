extern crate gl;

use gl::types::{GLchar, GLint, GLuint};
use std::ffi::CString;

const INFO_BUFFER_CAPACITY: usize = 512;

pub struct Shader {
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
