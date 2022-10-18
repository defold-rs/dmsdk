use crate::{dmgameobject, dmvmath, ffi::dmScript, lua};

pub unsafe fn check_go_instance(l: lua::State) -> dmgameobject::Instance {
    dmScript::CheckGOInstance(l)
}

pub unsafe fn push_vector3(l: lua::State, v: dmvmath::Vector3) {
    dmScript::PushVector3(l, &v.into())
}
