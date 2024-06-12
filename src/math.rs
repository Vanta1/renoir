//! Re-exports of nalgebra types, centralized here so that we don't need to keep specifying f32
//! as the type parameter for each commonly used struct. This also helps to maintain consistency
//! across the codebase.
//! Another useful feature is that these are also re-exported in the crate prelude, making them
//! accessible to end users, where they will likely
//! be useful.

pub type Vec3 = nalgebra::Vector3<f32>;
pub type Point3 = nalgebra::Point3<f32>;
pub type Mat4 = nalgebra::Matrix4<f32>;
pub type Quat = nalgebra::Quaternion<f32>;
pub type UnitQuat = nalgebra::UnitQuaternion<f32>;
pub type Rot3 = nalgebra::Rotation3<f32>;
pub type Trans3 = nalgebra::Translation3<f32>;

pub mod prelude {
    pub use super::Mat4;
    pub use super::Point3;
    pub use super::Quat;
    pub use super::Rot3;
    pub use super::Trans3;
    pub use super::UnitQuat;
    pub use super::Vec3;
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
