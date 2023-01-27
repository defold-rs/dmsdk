use dmextension::ctor;
use dmsdk::{ffi::dmGameObject, *};

unsafe extern "C" fn create(params: *const dmGameObject::ComponentCreateParams) -> i32 {
    // let params = dmgameobject::ComponentCreateParams::from(params);
    // println!("{:?}", params);

    dmgameobject::CreateResult::Ok as i32
}

pub fn create_type(
    ctx: *const dmGameObject::ComponentTypeCreateCtx,
    component: *mut dmGameObject::ComponentType,
) -> i32 {
    //component.set_create_fn(create);
    0
}
pub fn destroy_type(
    ctx: *const dmGameObject::ComponentTypeCreateCtx,
    component: *mut dmGameObject::ComponentType,
) -> i32 {
    //component.set_create_fn(create);
    0
}
