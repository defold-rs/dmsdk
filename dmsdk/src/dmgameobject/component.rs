use std::{ffi::CString, os::raw::c_void};

use dmsdk_ffi::dmGameObject;

use crate::dmvmath::{Point3, Quat, Vector3};

use super::Instance;

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
    pub position: Point3,
    pub rotation: Quat,
    pub scale: Vector3,
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
