//! Functions for manipulating game objects.

use std::ffi::CString;

use libc::c_void;

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
    destroy: Option<RawComponentTypeDestroyFn>,
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

impl ComponentCreateParams {
    pub fn new(ptr: *const dmGameObject::ComponentCreateParams) -> Self {
        unsafe { Self::from(ptr) }
    }
}

impl From<*const dmGameObject::ComponentCreateParams> for ComponentCreateParams {
    fn from(ptr: *const dmGameObject::ComponentCreateParams) -> Self {
        unsafe { Self::from(ptr) }
    }
}

impl From<&dmGameObject::ComponentCreateParams> for ComponentCreateParams {
    fn from(params: &dmGameObject::ComponentCreateParams) -> Self {
        Self {
            instance: params.m_Instance,
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
    ($symbol:ident, $create:expr, $destroy:expr) => {
		paste! {
			static mut [<$symbol _DESC>]: dmgameobject::ComponentDesc = [0u8; dmgameobject::DESC_BUFFER_SIZE];

			#[no_mangle]
			unsafe extern "C" fn [<$symbol _create>](ctx: *const ffi::dmGameObject::ComponentTypeCreateCtx, component: *mut ffi::dmGameObject::ComponentType) -> i32 {
				$create(ctx, dmgameobject::ComponentType::new(component))
			}

			#[no_mangle]
			unsafe extern "C" fn [<$symbol _destroy>](ctx: *const ffi::dmGameObject::ComponentTypeCreateCtx, component: *mut ffi::dmGameObject::ComponentType) -> i32 {
				0
			}

			#[no_mangle]
			#[ctor]
			unsafe fn $symbol() {
				dmgameobject::register_component_type(
					stringify!($symbol),
					&mut [<$symbol _DESC>],
					[<$symbol _create>],
					Some([<$symbol _destroy>]),
				);
			}
		}
	};
}
