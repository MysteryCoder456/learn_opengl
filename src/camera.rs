pub struct Camera {
    position: nalgebra_glm::Vec3,
    front: nalgebra_glm::Vec3,
    up: nalgebra_glm::Vec3,
    fov: f32,
    // Euler Angles
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new(position: nalgebra_glm::Vec3, fov: f32, yaw: f32, pitch: f32) -> Self {
        let mut cam = Self {
            position,
            front: nalgebra_glm::Vec3::zeros(),
            up: nalgebra_glm::Vec3::y(),
            fov,
            yaw,
            pitch,
        };
        cam.update_vectors();
        cam
    }

    pub fn position(&self) -> nalgebra_glm::Vec3 {
        self.position
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn look_at_matrix(&self) -> nalgebra_glm::Mat4 {
        nalgebra_glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn look_at_custom(&self) -> nalgebra_glm::Mat4 {
        let right = self.front.cross(&self.up).normalize();
        let direction = -self.front;
        let up = direction.cross(&right).normalize();

        let rotation = nalgebra_glm::mat4(
            right.x,
            right.y,
            right.z,
            0.0,
            up.x,
            up.y,
            up.z,
            0.0,
            direction.x,
            direction.y,
            direction.z,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        let translate = nalgebra_glm::translate(&nalgebra_glm::identity(), &(-self.position));

        rotation * translate
    }

    pub fn move_front(&mut self, speed: f32) {
        self.position += self.front * speed;
    }

    pub fn move_side(&mut self, speed: f32) {
        self.position += self.front.cross(&self.up).normalize() * speed;
    }

    pub fn look_around(&mut self, dx: f32, dy: f32) {
        self.yaw += dx;
        self.pitch = (self.pitch + dy).clamp(-89.0, 89.0);
        self.update_vectors();
    }

    pub fn zoom(&mut self, d_fov: f32) {
        self.fov += d_fov;
        self.fov = self.fov.clamp(5.0, 120.0);
    }

    fn update_vectors(&mut self) {
        let cos_pitch = self.pitch.to_radians().cos();
        let direction = nalgebra_glm::vec3(
            self.yaw.to_radians().cos() * cos_pitch,
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * cos_pitch,
        );
        self.front = direction.normalize();
    }
}
