use cgmath::{Deg, Euler, Quaternion, Vector3};
use wgpu::SurfaceConfiguration;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(config: &SurfaceConfiguration) -> Self {
        Self {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

pub struct CameraController {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub rotation: Quaternion<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            rotation: (0.0, 1.0, 1.0, 0.0).into(),
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.target = self.eye + (Quaternion::from(Euler::new(Deg(self.yaw), Deg(self.pitch), Deg(0.0))) * Vector3::unit_z());
    }

    pub fn rotate(&mut self, deg_x: f32, deg_y: f32, deg_z: f32) {
        self.yaw += deg_y;
        self.pitch += deg_x;
        self.roll += deg_z;
    }
}
