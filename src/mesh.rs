use std::os::raw::c_void;

use gl::types::GLuint;
use nalgebra_glm as glm;

use crate::Shader;

#[repr(packed)]
pub struct Vertex {
    position: glm::Vec3,
    normal: glm::Vec3,
    texture_coords: glm::Vec2,
}

pub enum TextureType {
    Diffuse,
    Specular,
}

pub struct Texture {
    id: GLuint,
    texture_type: TextureType,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<usize>,
    pub textures: Vec<Texture>,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

impl Mesh {
    pub unsafe fn new(vertices: Vec<Vertex>, indices: Vec<usize>, textures: Vec<Texture>) -> Self {
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

        // Create vertex array
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create vertex buffer
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Create element buffer
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&indices) as isize,
            indices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        let stride = 8 * std::mem::size_of::<f32>();

        // Position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // Normal attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as i32,
            (3 * std::mem::size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        // Texture coordinate attribute
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride as i32,
            (6 * std::mem::size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);

        // Unbind vertex array
        gl::BindVertexArray(0);

        Self {
            vertices,
            indices,
            textures,
            vao,
            vbo,
            ebo,
        }
    }

    pub unsafe fn draw(&self, shader: &Shader) {
        shader.use_program();

        // Bind textures
        for (i, texture) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);

            let type_str = match texture.texture_type {
                TextureType::Diffuse => "texture_diffuse",
                TextureType::Specular => "texture_specular",
            };
            gl::Uniform1f(
                shader.get_uniform_location(&format!("material.{type_str}{i}")),
                i as f32,
            );
        }

        // Draw elements
        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as i32,
            gl::UNSIGNED_INT,
            0 as *const c_void,
        );
        gl::BindVertexArray(0);
    }
}
