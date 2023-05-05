//! Defold-specific Lua helpers.

use dmsdk_ffi::dmScript;

use crate::{dmgameobject, dmvmath, lua};

/// Returns the game object instance the calling script belongs to.
pub fn check_go_instance(l: lua::State) -> dmgameobject::Instance {
    unsafe { dmgameobject::Instance::new(dmScript::CheckGOInstance(l.ptr())) }
}

/// Pushes a [`Vector3`](dmvmath::Vector3) onto the stack.
pub fn push_vector3(l: lua::State, v: dmvmath::Vector3) {
    unsafe { dmScript::PushVector3(l.ptr(), &v.into()) }
}
