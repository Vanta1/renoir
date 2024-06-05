use ultraviolet::{projection, Mat4, Rotor3, Vec3, Vec4};
use wgpu::SurfaceConfiguration;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::new(
    Vec4::new(1.0, 0.0, 0.0, 0.0),
    Vec4::new(0.0, 1.0, 0.0, 0.0),
    Vec4::new(0.0, 0.0, 0.5, 0.5),
    Vec4::new(0.0, 0.0, 0.0, 1.0),
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    up: Vec3,
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
            up: Vec3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at(self.eye, self.target, self.up);
        let proj = projection::rh_yup::perspective_wgpu_dx(self.fovy, self.aspect, self.znear, self.zfar);
        return proj * view;
    }
}

pub struct CameraController {
    pub eye: Vec3,
    pub target: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.target = self.eye + (Rotor3::from_euler_angles(self.yaw, self.pitch, self.roll) * Vec3::unit_z());
    }

    pub fn rotate(&mut self, deg_x: f32, deg_y: f32, deg_z: f32) {
        self.yaw -= deg_y;
        self.pitch -= deg_x;
        self.roll += deg_z;
    }

    pub fn set_translate(&mut self, mag_x: f32, mag_y: f32, mag_z: f32) {
        self.eye = (mag_x, mag_y, mag_z).into();
    } 
}
