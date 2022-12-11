//! Defold-specific Lua helpers.

use crate::{dmgameobject, dmvmath, ffi::dmScript, lua};

/// Returns the game object instance the calling script belongs to.
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn check_go_instance(l: lua::State) -> dmgameobject::Instance {
    dmScript::CheckGOInstance(l)
}

/// Pushes a [`Vector3`](dmvmath::Vector3) onto the stack.
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn push_vector3(l: lua::State, v: dmvmath::Vector3) {
    dmScript::PushVector3(l, &v.into())
}
