use crate::{dmgameobject, ffi::dmScript, lua};

pub unsafe fn check_go_instance(l: lua::State) -> dmgameobject::Instance {
    dmScript::CheckGOInstance(l)
}
