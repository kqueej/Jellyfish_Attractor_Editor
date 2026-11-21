use super::camera::OrbitCamera;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_projectrion: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new(camera: &OrbitCamera, aspect: f32) -> Self {
        Self {
            view_projectrion: camera.view_projection_matrix(aspect).to_cols_array_2d(),
        }
    }
}
