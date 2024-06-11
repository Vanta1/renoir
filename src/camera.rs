use wgpu::SurfaceConfiguration;

use crate::math::OPENGL_TO_WGPU_MATRIX;
use crate::math::prelude::*;

pub enum TransformSpace {
    Local,
    World,
}

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
    pub eye: Point3,
    pub target: Point3,
    up: Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(config: &SurfaceConfiguration) -> Self {
        Self {
            eye: Point3::new(0.0, 0.0, 10.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vec3::y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 90.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(&self.eye, &self.target, &self.up);
        let proj = Mat4::new_perspective(self.aspect, self.fovy, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

pub struct CameraController {
    pub pos: Point3,
    pub target: Point3,
    iso: Mat4,
    rot: UnitQuat, 
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            pos: Point3::new(0.0, 1.0, -2.0),
            target: Point3::new(0.0, 0.0, 0.0),
            iso: Mat4::identity(),
            rot: UnitQuat::identity(),
        }
    }

    // TODO: move this into rotate_around_axis prob, and get rid of the 'iso' field.
    fn rebuild_iso(&mut self) {
        self.iso = (Trans3::new(self.pos.x, self.pos.y, self.pos.z) * Rot3::from(self.rot).transpose()).to_matrix();
    }

    pub fn update(&mut self) {
        self.target = self.pos + (Rot3::from(self.rot).transpose() * Vec3::z());
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.rotate_around_axis(Vec3::x().xyz(), angle, TransformSpace::Local);
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.rotate_around_axis(Vec3::y().xyz(), angle, TransformSpace::World);
    }

    pub fn rotate_around_axis(&mut self, axis: Vec3, angle: f32, space: TransformSpace) {
        let axis = axis.normalize();
        let axis = match space {
            TransformSpace::Local => axis,
            TransformSpace::World => self.iso.try_inverse().unwrap().transform_vector(&axis),
        };
        self.rot = UnitQuat::from_scaled_axis(axis * angle) * self.rot;
        self.rebuild_iso();
        self.update();
    }

    pub fn set_translate(&mut self, x: f32, y: f32, z: f32) {
        self.pos.x = x;
        self.pos.y = y;
        self.pos.z = z;
        self.rebuild_iso();
    }
}
