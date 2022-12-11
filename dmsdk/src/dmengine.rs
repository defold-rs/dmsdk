use crate::{dmconfigfile, dmextension, dmgameobject, dmwebserver, ffi::dmEngine};

pub fn get_config_file(app_params: dmextension::AppParams) -> dmconfigfile::ConfigFile {
    unsafe { dmEngine::GetConfigFile(app_params.ptr) }
}

pub fn get_web_server(app_params: dmextension::AppParams) -> dmwebserver::Server {
    unsafe { dmEngine::GetWebServer(app_params.ptr) }
}

pub fn get_game_object_register(app_params: dmextension::AppParams) -> dmgameobject::Register {
    unsafe { dmEngine::GetGameObjectRegister(app_params.ptr) }
}
