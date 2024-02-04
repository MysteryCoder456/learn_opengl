// Getting Started - Section 9: Coordinate Systems

extern crate gl;
extern crate glfw;

use std::os::raw::c_void;

use gl::types::GLfloat;
use glfw::Context;
use image::io::Reader as ImageReader;
use learn_opengl::Shader;

#[rustfmt::skip]
const TRIANGLE_VERTICES: [f32; 40] = [
    //    position   |  texture coords
    -0.5,  0.5, -0.5,     0.0, 1.0,
     0.5,  0.5, -0.5,     1.0, 0.0,
     0.5, -0.5, -0.5,     1.0, 1.0,
    -0.5, -0.5, -0.5,     0.0, 0.0,
    -0.5,  0.5,  0.5,     1.0, 1.0,
     0.5,  0.5,  0.5,     0.0, 0.0,
     0.5, -0.5,  0.5,     0.0, 1.0,
    -0.5, -0.5,  0.5,     1.0, 0.0,
];

#[rustfmt::skip]
const TRIANGLE_ELEMENTS: [u32; 36] = [
    0, 1, 2,
    0, 2, 3,
    4, 5, 6,
    4, 6, 7,
    0, 4, 5,
    0, 5, 1,
    0, 4, 7,
    0, 7, 3,
    3, 7, 6,
    3, 6, 2,
    1, 5, 6,
    1, 6, 2,
];

const CUBE_POSITIONS: [nalgebra_glm::Vec3; 10] = [
    nalgebra_glm::Vec3::new(0.0, 0.0, 0.0),
    nalgebra_glm::Vec3::new(2.0, 5.0, -15.0),
    nalgebra_glm::Vec3::new(-1.5, -2.2, -2.5),
    nalgebra_glm::Vec3::new(-3.8, -2.0, -12.3),
    nalgebra_glm::Vec3::new(2.4, -0.4, -3.5),
    nalgebra_glm::Vec3::new(-1.7, 3.0, -7.5),
    nalgebra_glm::Vec3::new(1.3, -2.0, -2.5),
    nalgebra_glm::Vec3::new(1.5, 2.0, -2.5),
    nalgebra_glm::Vec3::new(1.5, 0.2, -1.5),
    nalgebra_glm::Vec3::new(-1.3, 1.0, -1.5),
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

    // Enable depth testing
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let shader_program = unsafe {
        let shader = Shader::new(
            "shaders/section_09/vertex.glsl",
            "shaders/section_09/fragment.glsl",
        );

        if let Err(e) = shader {
            panic!("Failed to load shaders:\n{}", e);
        }
        shader.unwrap()
    };

    let wood_texture = unsafe {
        // Load texture image
        let img = ImageReader::open("assets/textures/container.jpg")
            .expect("Unable to open image file")
            .decode()
            .unwrap()
            .flipv();
        let img_bytes = img.as_bytes();

        // Generate and bind a new texture
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Configure texture wrapping and filtering options
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Set texture data and generate mipmaps
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            img_bytes.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        texture
    };

    let smile_texture = unsafe {
        // Load texture image
        let img = ImageReader::open("assets/textures/awesomeface.png")
            .unwrap()
            .decode()
            .unwrap()
            .flipv();
        let img_bytes = img.as_bytes();

        // Generate and bind a new texture
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Configure texture wrapping and filtering options
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Set texture data and generate mipmaps
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img_bytes.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        texture
    };

    let vao = unsafe {
        // Create a vertex buffer, element buffer and vertex array object
        let (mut vbo, mut ebo, mut vao) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        // Bind the vertex array first
        gl::BindVertexArray(vao);

        // Bind a buffer object as a vertex buffer and add data to it
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&TRIANGLE_VERTICES) as isize,
            &TRIANGLE_VERTICES as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        // Bind a buffer object as an element buffer and add data to it
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&TRIANGLE_ELEMENTS) as isize,
            &TRIANGLE_ELEMENTS as *const u32 as *const c_void,
            gl::STATIC_DRAW,
        );

        let stride = 5 * std::mem::size_of::<f32>() as i32;

        // Configure vertex position attributes
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // Configure texture coordinate attributes
        gl::VertexAttribPointer(
            1,
            2,
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

    // Set initial uniform values
    unsafe {
        // Configure texture sampler locations
        shader_program.use_program();
        gl::Uniform1i(shader_program.get_uniform_location("texture1"), 0);
        gl::Uniform1i(shader_program.get_uniform_location("texture2"), 1);
    }

    // Get uniform locations
    let (model_location, view_location, projection_location) = unsafe {
        (
            shader_program.get_uniform_location("model"),
            shader_program.get_uniform_location("view"),
            shader_program.get_uniform_location("projection"),
        )
    };

    while !window.should_close() {
        process_events(&mut window, &events);

        // View transform
        let mut view = nalgebra_glm::Mat4::identity();
        view = nalgebra_glm::translate(&view, &nalgebra_glm::vec3(0.0, 0.0, -3.0));

        // Projection transform
        let window_size = window.get_size();
        let projection = nalgebra_glm::perspective(
            window_size.0 as f32 / window_size.1 as f32,
            (45.0 as f32).to_radians(),
            0.1,
            100.0,
        );

        // Rendering commands
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Use our shader program
            shader_program.use_program();

            // Set transform uniform locations
            gl::UniformMatrix4fv(
                view_location,
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&view).as_ptr() as *const GLfloat,
            );
            gl::UniformMatrix4fv(
                projection_location,
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&projection).as_ptr() as *const GLfloat,
            );

            // Bind wood container texture first
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, wood_texture);

            // Bind smile texture second
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, smile_texture);

            // Bind vertex array
            gl::BindVertexArray(vao);

            for (i, pos) in CUBE_POSITIONS.iter().enumerate() {
                // Create model transform
                let mut model = nalgebra_glm::Mat4::identity();
                model = nalgebra_glm::translate(&model, &pos);
                model = nalgebra_glm::rotate(
                    &model,
                    (20 * i) as f32,
                    &(nalgebra_glm::vec3(0.5, 1.0, 0.0)),
                );

                // Set model transform
                gl::UniformMatrix4fv(
                    model_location,
                    1,
                    gl::FALSE,
                    nalgebra_glm::value_ptr(&model).as_ptr() as *const GLfloat,
                );

                // Draw cube
                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, 0 as *const c_void);
            }
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
