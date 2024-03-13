// Lighting - Section 14: Materials - Exercise 1

extern crate gl;
extern crate glfw;

use std::ffi::c_void;

use glfw::{
    Action, Context, CursorMode, GlfwReceiver, Key, OpenGlProfileHint, PWindow, WindowEvent,
    WindowHint, WindowMode,
};
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

const MOUSE_SENSITIVITY: f32 = 0.2;
const CAMERA_SPEED: f32 = 4.0;

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
            "shaders/section_14_exercise_1/cube_vert.glsl",
            "shaders/section_14_exercise_1/light_frag.glsl",
        )
        .unwrap();
        let cube_shader = Shader::new(
            "shaders/section_14_exercise_1/cube_vert.glsl",
            "shaders/section_14_exercise_1/cube_frag.glsl",
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

    let mut camera = Camera::new(glm::vec3(0.0, 1.0, 3.0), 60.0, -90.0, -10.0);

    let light_pos = glm::vec3(2.0, 1.5, -5.0);
    let cube_pos = glm::vec3(-1.0, -1.0, -1.0);

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

        let light_color = glm::vec3(
            (now * 2.0).sin() as f32,
            (now * 0.7).sin() as f32,
            (now * 1.3).sin() as f32,
        );
        let ambient_light = light_color * 0.1;
        let diffuse_light = light_color * 0.7;

        let mut light_model = glm::Mat4::identity();
        light_model = glm::translate(&light_model, &light_pos);

        let mut cube_model = glm::Mat4::identity();
        cube_model = glm::translate(&cube_model, &cube_pos);

        let view = camera.look_at_matrix();

        let window_size = window.get_size();
        let projection = glm::perspective(
            window_size.0 as f32 / window_size.1 as f32,
            camera.fov().to_radians(),
            0.1,
            100.0,
        );

        // Light cube uniforms
        unsafe {
            light_shader.use_program();

            gl::Uniform3fv(
                light_shader.get_uniform_location("lightColor"),
                1,
                glm::value_ptr(&diffuse_light).as_ptr(),
            );

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
            gl::Uniform3f(
                cube_shader.get_uniform_location("material.ambient"),
                1.0,
                0.5,
                0.31,
            );
            gl::Uniform3f(
                cube_shader.get_uniform_location("material.diffuse"),
                1.0,
                0.5,
                0.31,
            );
            gl::Uniform3f(
                cube_shader.get_uniform_location("material.specular"),
                0.5,
                0.5,
                0.5,
            );
            gl::Uniform1f(cube_shader.get_uniform_location("material.shininess"), 32.0);

            // Light
            gl::Uniform3fv(
                cube_shader.get_uniform_location("light.position"),
                1,
                glm::value_ptr(&light_pos).as_ptr(),
            );
            gl::Uniform3fv(
                cube_shader.get_uniform_location("light.ambient"),
                1,
                glm::value_ptr(&ambient_light).as_ptr(),
            );
            gl::Uniform3fv(
                cube_shader.get_uniform_location("light.diffuse"),
                1,
                glm::value_ptr(&diffuse_light).as_ptr(),
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
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
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
