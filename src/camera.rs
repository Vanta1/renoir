use nalgebra::{Matrix4, Perspective3, Point3, Rotation3, Vector, Vector3};
use wgpu::SurfaceConfiguration;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct Camera {
    pub eye: Vector3<f32>,
    pub target: Vector3<f32>,
    up: Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(config: &SurfaceConfiguration) -> Self {
        Self {
            eye: Vector3::new(0.0, 0.0, 2.0),
            target: Vector3::new(0.0, 0.0, 0.0),
            up: Vector3::y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 90.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(&self.eye.into(), &self.target.into(), &self.up);
        let proj = Matrix4::new_perspective(self.fovy, self.aspect, self.znear, self.zfar);
        return proj * view;
    }
}

pub struct CameraController {
    pub eye: Vector3<f32>,
    pub target: Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            eye: Vector3::new(0.0, 1.0, 2.0),
            target: Vector3::new(0.0, 0.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.target = self.eye + (Rotation3::from_euler_angles(self.yaw, self.pitch, self.roll) * Vector3::z());
    }

    pub fn rotate(&mut self, deg_x: f32, deg_y: f32, deg_z: f32) {
        self.yaw -= deg_y;
        self.pitch -= deg_x;
        self.roll += deg_z;
    }

    pub fn set_translate(&mut self, x: f32, y: f32, z: f32) {
        self.eye.x = x;
        self.eye.y = y;
        self.eye.z = z;
    }
}
