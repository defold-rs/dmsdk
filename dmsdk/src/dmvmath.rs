//! Vector math helpers.

use crate::ffi::Vectormath;

/// Point in 3D space.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Vector in 3D space.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Quaternion.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// Vector3 <-> Point3 //
impl From<Vector3> for Point3 {
    fn from(v: Vector3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Point3> for Vector3 {
    fn from(p: Point3) -> Self {
        Self {
            x: p.x,
            y: p.y,
            z: p.z,
        }
    }
}

// Defold <-> Rust //
impl From<Vectormath::Aos::Point3> for Point3 {
    fn from(p: Vectormath::Aos::Point3) -> Self {
        Self {
            x: p.mX,
            y: p.mY,
            z: p.mZ,
        }
    }
}

#[cfg(target_env = "gnu")]
impl From<Point3> for Vectormath::Aos::Point3 {
    fn from(p: Point3) -> Self {
        Self {
            mX: p.x,
            mY: p.y,
            mZ: p.z,
            ..Default::default()
        }
    }
}

#[cfg(not(target_env = "gnu"))]
impl From<Point3> for Vectormath::Aos::Point3 {
    fn from(p: Point3) -> Self {
        Self {
            mX: p.x,
            mY: p.y,
            mZ: p.z,
            d: 0.0,
        }
    }
}

impl From<Vectormath::Aos::Vector3> for Vector3 {
    fn from(v: Vectormath::Aos::Vector3) -> Self {
        Self {
            x: v.mX,
            y: v.mY,
            z: v.mZ,
        }
    }
}

#[cfg(target_env = "gnu")]
impl From<Vector3> for Vectormath::Aos::Vector3 {
    fn from(v: Vector3) -> Self {
        Self {
            mX: v.x,
            mY: v.y,
            mZ: v.z,
            ..Default::default()
        }
    }
}

#[cfg(not(target_env = "gnu"))]
impl From<Vector3> for Vectormath::Aos::Vector3 {
    fn from(v: Vector3) -> Self {
        Self {
            mX: v.x,
            mY: v.y,
            mZ: v.z,
            d: 0.0,
        }
    }
}

impl From<Vectormath::Aos::Quat> for Quat {
    fn from(q: Vectormath::Aos::Quat) -> Self {
        Self {
            x: q.mX,
            y: q.mY,
            z: q.mZ,
            w: q.mW,
        }
    }
}
