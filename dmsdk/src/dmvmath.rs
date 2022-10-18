use crate::ffi::Vectormath;

#[derive(Debug)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
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

impl From<Point3> for Vectormath::Aos::Point3 {
    fn from(p: Point3) -> Self {
        Self {
            mX: p.x,
            mY: p.y,
            mZ: p.z,
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

impl From<Vector3> for Vectormath::Aos::Vector3 {
    fn from(v: Vector3) -> Self {
        Self {
            mX: v.x,
            mY: v.y,
            mZ: v.z,
        }
    }
}

impl<'a> From<&'a Vector3> for Vectormath::Aos::Vector3 {
    fn from(v: &'a Vector3) -> Self {
        Self {
            mX: v.x,
            mY: v.y,
            mZ: v.z,
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
