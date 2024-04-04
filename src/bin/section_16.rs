// Lighting - Section 16: Light Casters

extern crate gl;
extern crate glfw;

use std::ffi::c_void;

use glfw::{
    Action, Context, CursorMode, GlfwReceiver, Key, OpenGlProfileHint, PWindow, WindowEvent,
    WindowHint, WindowMode,
};
use image::io::Reader as ImageReader;
use learn_opengl::{Camera, Shader};
use nalgebra_glm as glm;

#[rustfmt::skip]
const CUBE_VERTICES: [f32; 288] = [
    //    position    |      normals     |  texture  
    -0.5, -0.5, -0.5,   0.0,  0.0, -1.0,   0.0, 0.0, 
     0.5, -0.5, -0.5,   0.0,  0.0, -1.0,   1.0, 0.0, 
     0.5,  0.5, -0.5,   0.0,  0.0, -1.0,   1.0, 1.0, 
     0.5,  0.5, -0.5,   0.0,  0.0, -1.0,   1.0, 1.0, 
    -0.5,  0.5, -0.5,   0.0,  0.0, -1.0,   0.0, 1.0, 
    -0.5, -0.5, -0.5,   0.0,  0.0, -1.0,   0.0, 0.0, 
                                         
    -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,   0.0, 0.0, 
     0.5, -0.5,  0.5,   0.0,  0.0,  1.0,   1.0, 0.0, 
     0.5,  0.5,  0.5,   0.0,  0.0,  1.0,   1.0, 1.0, 
     0.5,  0.5,  0.5,   0.0,  0.0,  1.0,   1.0, 1.0, 
    -0.5,  0.5,  0.5,   0.0,  0.0,  1.0,   0.0, 1.0, 
    -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,   0.0, 0.0, 
                                         
    -0.5,  0.5,  0.5,  -1.0,  0.0,  0.0,   1.0, 0.0, 
    -0.5,  0.5, -0.5,  -1.0,  0.0,  0.0,   1.0, 1.0, 
    -0.5, -0.5, -0.5,  -1.0,  0.0,  0.0,   0.0, 1.0, 
    -0.5, -0.5, -0.5,  -1.0,  0.0,  0.0,   0.0, 1.0, 
    -0.5, -0.5,  0.5,  -1.0,  0.0,  0.0,   0.0, 0.0, 
    -0.5,  0.5,  0.5,  -1.0,  0.0,  0.0,   1.0, 0.0, 
                                         
     0.5,  0.5,  0.5,   1.0,  0.0,  0.0,   1.0, 0.0, 
     0.5,  0.5, -0.5,   1.0,  0.0,  0.0,   1.0, 1.0, 
     0.5, -0.5, -0.5,   1.0,  0.0,  0.0,   0.0, 1.0, 
     0.5, -0.5, -0.5,   1.0,  0.0,  0.0,   0.0, 1.0, 
     0.5, -0.5,  0.5,   1.0,  0.0,  0.0,   0.0, 0.0, 
     0.5,  0.5,  0.5,   1.0,  0.0,  0.0,   1.0, 0.0, 
                                         
    -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,   0.0, 1.0, 
     0.5, -0.5, -0.5,   0.0, -1.0,  0.0,   1.0, 1.0, 
     0.5, -0.5,  0.5,   0.0, -1.0,  0.0,   1.0, 0.0, 
     0.5, -0.5,  0.5,   0.0, -1.0,  0.0,   1.0, 0.0, 
    -0.5, -0.5,  0.5,   0.0, -1.0,  0.0,   0.0, 0.0, 
    -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,   0.0, 1.0, 
                                         
    -0.5,  0.5, -0.5,   0.0,  1.0,  0.0,   0.0, 1.0, 
     0.5,  0.5, -0.5,   0.0,  1.0,  0.0,   1.0, 1.0, 
     0.5,  0.5,  0.5,   0.0,  1.0,  0.0,   1.0, 0.0, 
     0.5,  0.5,  0.5,   0.0,  1.0,  0.0,   1.0, 0.0, 
    -0.5,  0.5,  0.5,   0.0,  1.0,  0.0,   0.0, 0.0, 
    -0.5,  0.5, -0.5,   0.0,  1.0,  0.0,   0.0, 1.0, 
];
const CUBE_POSITIONS: [glm::Vec3; 10] = [
    glm::Vec3::new(0.0, 0.0, 0.0),
    glm::Vec3::new(2.0, 5.0, -15.0),
    glm::Vec3::new(-1.5, -2.2, -2.5),
    glm::Vec3::new(-3.8, -2.0, -12.3),
    glm::Vec3::new(2.4, -0.4, -3.5),
    glm::Vec3::new(-1.7, 3.0, -7.5),
    glm::Vec3::new(1.3, -2.0, -2.5),
    glm::Vec3::new(1.5, 2.0, -2.5),
    glm::Vec3::new(1.5, 0.2, -1.5),
    glm::Vec3::new(-1.3, 1.0, -1.5),
];

const MOUSE_SENSITIVITY: f32 = 0.2;
const CAMERA_SPEED: f32 = 5.0;

unsafe fn load_texture<P>(file_path: P) -> u32
where
    P: AsRef<std::path::Path>,
{
    let img = ImageReader::open(file_path)
        .unwrap()
        .decode()
        .unwrap()
        .flipv();
    let img_bytes = img.as_bytes();

    // Create texture
    let mut texture = 0;
    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture);

    // Texture parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    // Set texture pixel data
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
}

fn process_events(
    events: &GlfwReceiver<(f64, WindowEvent)>,
    window: &mut PWindow,
    delta_time: f32,
    last_mouse_x: &mut f32,
    last_mouse_y: &mut f32,
    camera: &mut Camera,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            WindowEvent::FramebufferSize(w, h) => unsafe {
                gl::Viewport(0, 0, w, h);
            },
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            WindowEvent::CursorPos(x_pos, y_pos) => {
                let dx = (x_pos as f32 - *last_mouse_x) * MOUSE_SENSITIVITY;
                let dy = -(y_pos as f32 - *last_mouse_y) * MOUSE_SENSITIVITY;

                *last_mouse_x = x_pos as f32;
                *last_mouse_y = y_pos as f32;

                camera.look_around(dx, dy);
            }
            WindowEvent::Scroll(_, offset) => camera.zoom(offset as f32),
            _ => {}
        }
    }

    if window.get_key(glfw::Key::W) == glfw::Action::Press {
        camera.move_front(CAMERA_SPEED * delta_time);
    } else if window.get_key(glfw::Key::S) == glfw::Action::Press {
        camera.move_front(-CAMERA_SPEED * delta_time);
    } else if window.get_key(glfw::Key::A) == glfw::Action::Press {
        camera.move_side(-CAMERA_SPEED * delta_time);
    } else if window.get_key(glfw::Key::D) == glfw::Action::Press {
        camera.move_side(CAMERA_SPEED * delta_time);
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
    window.set_cursor_pos_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);
    window.set_scroll_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s));

    // Enable OpenGL features
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let (light_shader, cube_shader) = unsafe {
        let light_shader = Shader::new(
            "shaders/section_16/cube_vert.glsl",
            "shaders/section_16/light_frag.glsl",
        )
        .unwrap();
        let cube_shader = Shader::new(
            "shaders/section_16/cube_vert.glsl",
            "shaders/section_16/cube_frag.glsl",
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
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * f32_size, std::ptr::null());
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
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * f32_size, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // Surface normal coords
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * f32_size,
            (3 * f32_size) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        // Texture coords
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            8 * f32_size,
            (6 * f32_size) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);

        gl::BindVertexArray(0);
        vao
    };

    let diffuse_map = unsafe { load_texture("assets/textures/container2.png") };
    let specular_map = unsafe { load_texture("assets/textures/container2_specular.png") };

    let mut camera = Camera::new(glm::vec3(0.0, 1.0, 3.0), 60.0, -90.0, -10.0);
    let light_pos = glm::vec3(0.0, 0.0, -6.0);

    let mut last_mouse_x = 0.0;
    let mut last_mouse_y = 0.0;

    let mut last_frame = glfw.get_time();

    while !window.should_close() {
        let now = glfw.get_time();
        let delta_time = (now - last_frame) as f32;
        last_frame = now;

        // Process window events
        process_events(
            &events,
            &mut window,
            delta_time,
            &mut last_mouse_x,
            &mut last_mouse_y,
            &mut camera,
        );

        let view = camera.look_at_matrix();

        let window_size = window.get_size();
        let projection = glm::perspective(
            window_size.0 as f32 / window_size.1 as f32,
            camera.fov().to_radians(),
            0.1,
            100.0,
        );

        let mut light_model = glm::Mat4::identity();
        light_model = glm::translate(&light_model, &light_pos);
        light_model = glm::scale(&light_model, &glm::vec3(0.2, 0.2, 0.2));

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

            // Material
            gl::Uniform1i(cube_shader.get_uniform_location("material.diffuse"), 0);
            gl::Uniform1i(cube_shader.get_uniform_location("material.specular"), 1);
            gl::Uniform1f(cube_shader.get_uniform_location("material.shininess"), 32.0);

            // Light
            gl::Uniform3fv(
                cube_shader.get_uniform_location("light.position"),
                1,
                glm::value_ptr(&light_pos).as_ptr(),
            );
            gl::Uniform3f(
                cube_shader.get_uniform_location("light.ambient"),
                0.3,
                0.3,
                0.3,
            );
            gl::Uniform3f(
                cube_shader.get_uniform_location("light.diffuse"),
                0.8,
                0.8,
                0.8,
            );
            gl::Uniform3f(
                cube_shader.get_uniform_location("light.specular"),
                1.0,
                1.0,
                1.0,
            );

            gl::Uniform3fv(
                cube_shader.get_uniform_location("cameraPos"),
                1,
                glm::value_ptr(&camera.position()).as_ptr(),
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
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Draw light source
            light_shader.use_program();
            gl::BindVertexArray(light_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // Draw party cubes
            cube_shader.use_program();

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, diffuse_map);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, specular_map);

            gl::BindVertexArray(cube_vao);

            for (i, pos) in CUBE_POSITIONS.iter().enumerate() {
                let angle = 20.0 * i as f32 + now as f32 * 25.0;

                // Party cube transforms
                let mut cube_model = glm::Mat4::identity();
                cube_model = glm::translate(&cube_model, &pos);
                cube_model = glm::rotate(
                    &cube_model,
                    angle.to_radians(),
                    &glm::vec3(0.2, 0.7, 0.5).normalize(),
                );

                // Set uniform value
                gl::UniformMatrix4fv(
                    cube_shader.get_uniform_location("model"),
                    1,
                    gl::FALSE,
                    glm::value_ptr(&cube_model).as_ptr(),
                );

                // Draw the party cube
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        // Poll events and swap buffers
        window.swap_buffers();
        glfw.poll_events();
    }
}
