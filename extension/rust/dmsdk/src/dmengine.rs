use crate::{dmconfigfile, dmextension, dmgameobject, dmwebserver, ffi::dmEngine};

pub unsafe fn get_config_file(app_params: dmextension::AppParams) -> dmconfigfile::ConfigFile {
    dmEngine::GetConfigFile(app_params)
}

pub unsafe fn get_web_server(app_params: dmextension::AppParams) -> dmwebserver::Server {
    dmEngine::GetWebServer(app_params)
}

pub unsafe fn get_game_object_register(
    app_params: dmextension::AppParams,
) -> dmgameobject::Register {
    dmEngine::GetGameObjectRegister(app_params)
}
