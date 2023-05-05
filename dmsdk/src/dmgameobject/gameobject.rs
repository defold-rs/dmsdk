use std::fmt::Debug;

use crate::dmvmath::{Point3, Quat, Vector3};

use dmsdk_ffi::dmGameObject;

/// Game object register.
pub type Register = dmGameObject::HRegister;

const UNNAMED_IDENTIFIER: u64 = 12415623704795185700;

#[allow(missing_docs)]
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

/// Collection instance.
pub struct Collection {
    ptr: dmGameObject::HCollection,
}

impl From<dmGameObject::HCollection> for Collection {
    fn from(ptr: dmGameObject::HCollection) -> Self {
        Self { ptr }
    }
}

/// Game object instance.
pub struct Instance {
    ptr: dmGameObject::HInstance,
}

impl Instance {
    pub unsafe fn new(ptr: dmGameObject::HInstance) -> Self {
        Self { ptr }
    }

    /// Returns the ID of this game object, if it has one.
    pub fn id(&self) -> Option<u64> {
        let hash = unsafe { dmGameObject::GetIdentifier(self.ptr) };
        if hash == UNNAMED_IDENTIFIER {
            None
        } else {
            Some(hash)
        }
    }

    /// Returns the collection that this game object belongs to.
    pub fn collection(&self) -> Collection {
        unsafe { dmGameObject::GetCollection(self.ptr).into() }
    }

    /// Sets the position of this game object.
    pub fn set_position(&self, position: Point3) {
        unsafe { dmGameObject::SetPosition(self.ptr, position.into()) }
    }

    /// Sets the rotation of this game object.
    pub fn set_rotation(&self, rotation: Quat) {
        unsafe { dmGameObject::SetRotation(self.ptr, rotation.into()) }
    }

    /// Sets the scale of this game object.
    pub fn set_scale(&self, scale: Vector3) {
        unsafe { dmGameObject::SetScale1(self.ptr, scale.into()) }
    }
}

impl Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Instance").field(&self.id()).finish()
    }
}

/// [`Result`](core::result::Result) alias with an error type of [`Error`].
pub type Result<T> = core::result::Result<T, Error>;
