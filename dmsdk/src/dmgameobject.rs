//! Functions for manipulating game objects.

use std::ffi::CString;

use crate::{dmvmath, ffi::dmGameObject};

/// Game object register.
pub type Register = dmGameObject::HRegister;
/// Game object instance.
pub type Instance = dmGameObject::HInstance;

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

/// [`Result`](core::result::Result) alias with an error type of [`Error`].
pub type Result<T> = core::result::Result<T, Error>;

/// Returns the ID of a game object if it has one.
///
/// # Safety
///
/// This function is safe as long as `instance` points to a valid game object.
pub unsafe fn get_identifier(instance: Instance) -> Option<u64> {
    let hash = dmGameObject::GetIdentifier(instance);
    if hash == UNNAMED_IDENTIFIER {
        None
    } else {
        Some(hash)
    }
}

/// Returns the position of a game object.
///
/// # Safety
///
/// This function is safe as long as `instance` points to a valid game object.
pub unsafe fn get_position(instance: Instance) -> dmvmath::Point3 {
    dmGameObject::GetPosition(instance).into()
}

/// Returns the rotation of a game object.
///
/// # Safety
///
/// This function is safe as long as `instance` points to a valid game object.
pub unsafe fn get_rotation(instance: Instance) -> dmvmath::Quat {
    dmGameObject::GetRotation(instance).into()
}

/// Returns the scale of a game object.
///
/// # Safety
///
/// This function is safe as long as `instance` points to a valid game object.
pub unsafe fn get_scale(instance: Instance) -> dmvmath::Vector3 {
    dmGameObject::GetScale(instance).into()
}

/// Sets the position of a game object.
///
/// # Safety
///
/// This function is safe as long as `instance` points to a valid game object.
pub unsafe fn set_position(instance: Instance, position: dmvmath::Point3) {
    dmGameObject::SetPosition(instance, position.into())
}

#[doc(hidden)]
pub const DESC_BUFFER_SIZE: usize = 128;

pub type ComponentDesc = [u8; DESC_BUFFER_SIZE];
type RawComponentCreateFn = unsafe extern "C" fn(
    *const dmGameObject::ComponentTypeCreateCtx,
    *mut dmGameObject::ComponentType,
) -> i32;
type RawComponentDestroyFn = unsafe extern "C" fn(
    *const dmGameObject::ComponentTypeCreateCtx,
    *mut dmGameObject::ComponentType,
) -> i32;

pub fn register_component_type(
    name: &str,
    desc: &mut ComponentDesc,
    create: RawComponentCreateFn,
    destroy: Option<RawComponentDestroyFn>,
) {
    let name = CString::new(name).unwrap();
    unsafe {
        dmGameObject::RegisterComponentTypeDescriptor(
            desc.as_mut_ptr() as *mut dmGameObject::ComponentTypeDescriptor,
            name.as_ptr(),
            Some(create),
            destroy,
        );
    }
}

#[macro_export]
macro_rules! declare_component_type {
    ($symbol:ident, $create:expr, $destroy:expr) => {
		paste! {
			static mut [<$symbol> _DESC]: dmgameobject::ComponentDesc = [0u8; dmgameobject::DESC_BUFFER_SIZE];

			#[no_mangle]
			#[ctor]
			unsafe fn $symbol() {
				dmgameobject::register_component_type(
					stringify!($symbol),
					&mut [<$symbol _DESC>],
					[<$symbol _create>],
					[<$symbol _destroy>],
				);
			}
		}
	};
}
