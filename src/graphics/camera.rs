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
        glam::camera::rh::proj::directx::perspective(self.fov_y, aspect, 0.1, 1000.0)
    }

    pub fn view_projection_matrix(&self, aspect: f32) -> Mat4 {
        self.projection_matrix(aspect) * self.view_matrix()
    }
}
