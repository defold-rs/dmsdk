//! Custom resource registering module.
#![allow(missing_docs)]

use crate::ffi::dmResource;
use std::ffi::CString;

pub use ctor::ctor;
use libc::c_void;

pub type TypeCreatorDesc = dmResource::TypeCreatorDesc;
pub type Factory = dmResource::HFactory;
pub type ResourceTypeRegisterContext = dmResource::ResourceTypeRegisterContext;
pub type ResourcePreloadParams = dmResource::ResourcePreloadParams;
pub type ResourceCreateParams = dmResource::ResourceCreateParams;
pub type ResourcePostCreateParams = dmResource::ResourcePostCreateParams;
pub type ResourceDestroyParams = dmResource::ResourceDestroyParams;
pub type ResourceRecreateParams = dmResource::ResourceRecreateParams;

pub type ResourceTypeRegister = unsafe extern "C" fn(ctx: *mut ResourceTypeRegisterContext) -> i32;
pub type ResourceTypeDeregister =
    unsafe extern "C" fn(ctx: *mut dmResource::ResourceTypeRegisterContext) -> i32;
pub type ResourcePreload = unsafe extern "C" fn(params: *const ResourcePreloadParams) -> i32;
pub type ResourceCreate = unsafe extern "C" fn(params: *const ResourceCreateParams) -> i32;
pub type ResourcePostCreate = unsafe extern "C" fn(params: *const ResourcePostCreateParams) -> i32;
pub type ResourceDestroy = unsafe extern "C" fn(params: *const ResourceDestroyParams) -> i32;
pub type ResourceRecreate = unsafe extern "C" fn(params: *const ResourceRecreateParams) -> i32;

#[macro_export]
macro_rules! declare_resource_type {
    ($symbol:ident, $name:expr, $register_fn:expr, $deregister_fn:expr) => {
        paste! {
            static mut [<$symbol _TYPE_CREATOR_DESC>]: [u8; 128] = [0u8; 128];

            #[no_mangle]
            #[dmextension::ctor]
            unsafe fn $symbol() {
                dmresource::_register_type_creator_desc(
                    &mut [<$symbol _TYPE_CREATOR_DESC>],
                    $name,
                    $register_fn,
                    $deregister_fn,
                );
            }
        }
    };
}

pub fn _register_type_creator_desc(
    desc: &mut [u8; 128],
    name: &str,
    register_fn: ResourceTypeRegister,
    deregister_fn: Option<ResourceTypeDeregister>,
) {
    let name = CString::new(name).unwrap();
    unsafe {
        dmResource::RegisterTypeCreatorDesc(
            desc.as_mut_ptr() as *mut dmResource::TypeCreatorDesc,
            128,
            name.as_ptr(),
            Some(register_fn),
            deregister_fn,
        );
    }
}

/// # Safety
///
/// This functions is safe as long as `extension` and `context` are valid pointers.
#[allow(clippy::too_many_arguments)]
pub unsafe fn register_type(
    factory: Factory,
    extension: *const i8,
    context: *mut c_void,
    preload_fn: Option<ResourcePreload>,
    create_fn: Option<ResourceCreate>,
    post_create_fn: Option<ResourcePostCreate>,
    destroy_fn: Option<ResourceDestroy>,
    recreate_fn: Option<ResourceRecreate>,
) -> i32 {
    dmResource::RegisterType(
        factory,
        extension,
        context,
        preload_fn,
        create_fn,
        post_create_fn,
        destroy_fn,
        recreate_fn,
    )
}
