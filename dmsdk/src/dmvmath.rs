//! Vector math helpers.

use dmsdk_ffi::dmVMath;

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
impl From<dmVMath::Point3> for Point3 {
    fn from(p: dmVMath::Point3) -> Self {
        Self {
            x: p.mX,
            y: p.mY,
            z: p.mZ,
        }
    }
}

#[cfg(target_env = "gnu")]
impl From<Point3> for dmVMath::Point3 {
    fn from(p: Point3) -> Self {
        Self {
            mX: p.x,
            mY: p.y,
            mZ: p.z,
        }
    }
}

#[cfg(not(target_env = "gnu"))]
impl From<Point3> for dmVMath::Point3 {
    fn from(p: Point3) -> Self {
        Self {
            mX: p.x,
            mY: p.y,
            mZ: p.z,
            d: 0.0,
        }
    }
}

impl From<dmVMath::Vector3> for Vector3 {
    fn from(v: dmVMath::Vector3) -> Self {
        Self {
            x: v.mX,
            y: v.mY,
            z: v.mZ,
        }
    }
}

#[cfg(target_env = "gnu")]
impl From<Vector3> for dmVMath::Vector3 {
    fn from(v: Vector3) -> Self {
        Self {
            mX: v.x,
            mY: v.y,
            mZ: v.z,
        }
    }
}

#[cfg(not(target_env = "gnu"))]
impl From<Vector3> for dmVMath::Vector3 {
    fn from(v: Vector3) -> Self {
        Self {
            mX: v.x,
            mY: v.y,
            mZ: v.z,
            d: 0.0,
        }
    }
}

impl From<dmVMath::Quat> for Quat {
    fn from(q: dmVMath::Quat) -> Self {
        Self {
            x: q.mX,
            y: q.mY,
            z: q.mZ,
            w: q.mW,
        }
    }
}

impl From<Quat> for dmVMath::Quat {
    fn from(q: Quat) -> Self {
        Self {
            mX: q.x,
            mY: q.y,
            mZ: q.z,
            mW: q.w,
        }
    }
}
