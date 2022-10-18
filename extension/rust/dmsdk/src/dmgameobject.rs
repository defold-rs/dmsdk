use crate::{dmvmath, ffi::dmGameObject};

pub type Register = dmGameObject::HRegister;
pub type Instance = dmGameObject::HInstance;

const UNNAMED_IDENTIFIER: u64 = 12415623704795185700;

pub enum Error {
    OutOfResources,
    AlreadyRegistered,
    IdentifierInUse,
    IdentifierAlreadySet,
    ComponentNotFound,
    MaximumHierarchicalDepth,
    InvalidOperation,
    ResourceTypeNotFound,
    BufferOverflow,
    Unknown,
}

impl From<i32> for Error {
    fn from(x: i32) -> Self {
        match x {
            -1 => Self::OutOfResources,
            -2 => Self::AlreadyRegistered,
            -3 => Self::IdentifierInUse,
            -4 => Self::IdentifierAlreadySet,
            -5 => Self::ComponentNotFound,
            -6 => Self::MaximumHierarchicalDepth,
            -7 => Self::InvalidOperation,
            -8 => Self::ResourceTypeNotFound,
            -9 => Self::BufferOverflow,
            _ => Self::Unknown,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

/// # Safety
pub unsafe fn get_identifier(instance: Instance) -> Option<u64> {
    let hash = dmGameObject::GetIdentifier(instance);
    if hash == UNNAMED_IDENTIFIER {
        None
    } else {
        Some(hash)
    }
}

/// # Safety
pub unsafe fn get_position(instance: Instance) -> dmvmath::Point3 {
    let pos = dmGameObject::GetPosition(instance);
    //println!("{:?}", pos);
    pos.into()
}

/// # Safety
pub unsafe fn get_rotation(instance: Instance) -> dmvmath::Quat {
    let rot = dmGameObject::GetRotation(instance);
    //println!("{:?}", rot);
    rot.into()
}

pub fn get_scale(instance: &Instance) -> dmvmath::Vector3 {
    unsafe { dmGameObject::GetScale(*instance) }.into()
}

/// # Safety
pub unsafe fn set_position(instance: Instance, position: dmvmath::Point3) {
    dmGameObject::SetPosition(instance, position.into())
}
