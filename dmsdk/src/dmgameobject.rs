//! Functions for manipulating game objects.

use std::{ffi::CString, fmt::Debug};

use dmsdk_ffi::dmVMath;
use libc::c_void;

use crate::{dmvmath, ffi::dmGameObject};

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

    /// Returns the position of this game object.
    pub fn position(&self) -> dmVMath::Point3 {
        unsafe { dmGameObject::GetPosition(self.ptr) }
    }

    /// Returns the rotation of this game object.
    pub fn rotation(&self) -> dmvmath::Quat {
        unsafe { dmGameObject::GetRotation(self.ptr).into() }
    }

    /// Returns the scale of this game object.
    pub fn scale(&self) -> dmvmath::Vector3 {
        unsafe { dmGameObject::GetScale(self.ptr).into() }
    }

    /// Sets the position of this game object.
    pub fn set_position(&self, position: dmvmath::Point3) {
        unsafe { dmGameObject::SetPosition(self.ptr, position.into()) }
    }
}

impl Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Instance").field(&self.id()).finish()
    }
}

/// [`Result`](core::result::Result) alias with an error type of [`Error`].
pub type Result<T> = core::result::Result<T, Error>;

#[doc(hidden)]
pub const DESC_BUFFER_SIZE: usize = 128;

pub type ComponentDesc = [u8; DESC_BUFFER_SIZE];
type RawComponentTypeCreateFn = unsafe extern "C" fn(
    *const dmGameObject::ComponentTypeCreateCtx,
    *mut dmGameObject::ComponentType,
) -> i32;
type RawComponentTypeDestroyFn = unsafe extern "C" fn(
    *const dmGameObject::ComponentTypeCreateCtx,
    *mut dmGameObject::ComponentType,
) -> i32;
pub type ComponentCreateFn = fn(ComponentCreateParams) -> CreateResult;
pub type RawComponentCreateFn =
    unsafe extern "C" fn(*const dmGameObject::ComponentCreateParams) -> i32;

pub enum CreateResult {
    Ok = 0,
    Err = -1000,
}

pub fn register_component_type(
    name: &str,
    desc: &mut ComponentDesc,
    create: RawComponentTypeCreateFn,
    destroy: RawComponentTypeDestroyFn,
) {
    let name = CString::new(name).unwrap();
    unsafe {
        dmGameObject::RegisterComponentTypeDescriptor(
            desc.as_mut_ptr() as *mut dmGameObject::ComponentTypeDescriptor,
            name.as_ptr(),
            Some(create),
            Some(destroy),
        );
    }
}

#[derive(Debug)]
pub struct PropertySet {}

impl From<dmGameObject::PropertySet> for PropertySet {
    fn from(_: dmGameObject::PropertySet) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct ComponentCreateParams {
    pub instance: Instance,
    pub position: dmvmath::Point3,
    pub rotation: dmvmath::Quat,
    pub scale: dmvmath::Vector3,
    pub property_set: PropertySet,
    pub resource: *mut c_void,
    pub world: *mut c_void,
    pub context: *mut c_void,
    pub user_data: *mut usize,
    pub index: u16,
}

impl From<*const dmGameObject::ComponentCreateParams> for ComponentCreateParams {
    fn from(ptr: *const dmGameObject::ComponentCreateParams) -> Self {
        let params = unsafe { *ptr };

        Self::from(&params)
    }
}

impl From<&dmGameObject::ComponentCreateParams> for ComponentCreateParams {
    fn from(params: &dmGameObject::ComponentCreateParams) -> Self {
        Self {
            instance: unsafe { Instance::new(params.m_Instance) },
            position: params.m_Position.into(),
            rotation: params.m_Rotation.into(),
            scale: params.m_Scale.into(),
            property_set: params.m_PropertySet.into(),
            resource: params.m_Resource,
            world: params.m_World,
            context: params.m_Context,
            user_data: params.m_UserData,
            index: params.m_ComponentIndex,
        }
    }
}

pub struct ComponentType {
    ptr: *mut dmGameObject::ComponentType,
}

impl ComponentType {
    pub fn new(ptr: *mut dmGameObject::ComponentType) -> Self {
        Self { ptr }
    }

    pub fn set_create_fn(&self, f: RawComponentCreateFn) {
        unsafe { dmGameObject::ComponentTypeSetCreateFn(self.ptr, Some(f)) }
    }
}

#[macro_export]
macro_rules! declare_component_type {
    ($symbol:ident, $name:expr, $create:expr, $destroy:expr) => {
		paste! {
			static mut [<$symbol _DESC>]: dmgameobject::ComponentDesc = [0u8; dmgameobject::DESC_BUFFER_SIZE];

			#[no_mangle]
			unsafe extern "C" fn [<$symbol _create>](ctx: *const ffi::dmGameObject::ComponentTypeCreateCtx, component: *mut ffi::dmGameObject::ComponentType) -> i32 {
				$create(ctx, component)
			}

			#[no_mangle]
			unsafe extern "C" fn [<$symbol _destroy>](ctx: *const ffi::dmGameObject::ComponentTypeCreateCtx, component: *mut ffi::dmGameObject::ComponentType) -> i32 {
				if let Some(func) = $destroy {
					func(ctx, component)
				} else {
					0
				}
			}

			#[no_mangle]
			#[ctor]
			unsafe fn $symbol() {
				dmgameobject::register_component_type(
					$name,
					&mut [<$symbol _DESC>],
					[<$symbol _create>],
					[<$symbol _destroy>],
				);
			}
		}
	};
}
