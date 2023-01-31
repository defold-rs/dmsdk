use std::ffi::c_void;

use dmsdk::{
    ffi::{dmGameObject, dmResource},
    *,
};

#[no_mangle]
unsafe extern "C" fn create_component(params: *const dmGameObject::ComponentCreateParams) -> i32 {
    let params = dmgameobject::ComponentCreateParams::from(params);
    println!("{:?}", params);

    dmgameobject::CreateResult::Ok as i32
}

pub fn create_type(
    ctx: *const dmGameObject::ComponentTypeCreateCtx,
    component: *mut dmGameObject::ComponentType,
) -> i32 {
    let component = dmgameobject::ComponentType::new(component);
    component.set_create_fn(create_component);

    0
}
pub fn destroy_type(
    ctx: *const dmGameObject::ComponentTypeCreateCtx,
    component: *mut dmGameObject::ComponentType,
) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn resource_create(params: *const dmResource::ResourceCreateParams) -> i32 {
    let params = *params;
    println!("{:?}", params);
    0
}

#[no_mangle]
pub unsafe extern "C" fn resource_destroy(params: *const dmResource::ResourceDestroyParams) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn resource_type_register(
    ctx: *mut dmResource::ResourceTypeRegisterContext,
) -> i32 {
    let ctx = *ctx;
    dmresource::register_type(
        ctx.m_Factory,
        ctx.m_Name,
        &mut () as *mut _ as *mut c_void,
        None,
        Some(resource_create),
        None,
        Some(resource_destroy),
        None,
    )
}
