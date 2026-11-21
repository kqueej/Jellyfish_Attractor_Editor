use glam::{Mat4, Vec3};

pub struct OrbitCamera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,
}

impl OrbitCamera {
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            target,
            distance,
            yaw: 90.0_f32.to_radians(),
            pitch: 20.0_f32.to_radians(),
            fov_y: 50.0_f32.to_radians(),
        }
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        const SENSITIVITY: f32 = 0.005;
        self.yaw -= delta_x * SENSITIVITY;
        self.pitch += delta_y * SENSITIVITY;
        let limit = 89.0_f32.to_radians();
        self.pitch = self.pitch.clamp(-limit, limit);
    }

    pub fn zoom(&mut self, delta: f32) {
        const ZOOM_SPEED: f32 = 0.15;
        const MIN_DISTANCE: f32 = 0.01;
        const MAX_DISTANCE: f32 = 20.0;

        let factor = (-delta * ZOOM_SPEED).exp();
        self.distance = (self.distance * factor).clamp(MIN_DISTANCE, MAX_DISTANCE);
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32, viewport_height: f32) {
        if viewport_height <= 0.0 {
            return;
        }

        let forward = (self.target - self.position()).normalize();
        let right = forward.cross(Vec3::Z).normalize();
        let up = right.cross(forward).normalize();
        let world_per_pixel = 2.0 * self.distance * (self.fov_y / 2.0).tan() / viewport_height;

        self.target += (-right * delta_x + up * delta_y) * world_per_pixel;
    }

    pub fn position(&self) -> Vec3 {
        let x = self.distance * self.pitch.cos() * self.yaw.cos();
        let y = self.distance * self.pitch.cos() * self.yaw.sin();
        let z = self.distance * self.pitch.sin();

        self.target + Vec3::new(x, y, z)
    }

    pub fn view_matrix(&self) -> Mat4 {
        glam::camera::rh::view::look_at_mat4(self.position(), self.target, Vec3::Z)
    }

    pub fn projection_matrix(&self, aspect: f32) -> Mat4 {
        glam::camera::rh::proj::directx::perspective(self.fov_y, aspect, 0.001, 1000.0)
    }

    pub fn view_projection_matrix(&self, aspect: f32) -> Mat4 {
        self.projection_matrix(aspect) * self.view_matrix()
    }
}
