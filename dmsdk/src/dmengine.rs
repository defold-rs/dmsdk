//! Functions for interacting with the engine.

use crate::{dmconfigfile, dmextension, dmgameobject, dmhid, dmwebserver, ffi::dmEngine};

/// Get the project config file from an instance of [`AppParams`](dmextension::AppParams).
pub fn get_config_file(app_params: dmextension::AppParams) -> dmconfigfile::ConfigFile {
    unsafe { dmEngine::GetConfigFile(app_params.ptr) }.into()
}

/// Get the web server from an instance of [`AppParams`](dmextension::AppParams).
pub fn get_web_server(app_params: dmextension::AppParams) -> dmwebserver::Server {
    unsafe { dmEngine::GetWebServer(app_params.ptr) }
}

/// Get the game object register from an instance of [`AppParams`](dmextension::AppParams).
pub fn get_game_object_register(app_params: dmextension::AppParams) -> dmgameobject::Register {
    unsafe { dmEngine::GetGameObjectRegister(app_params.ptr) }
}

/// Get the HID context from an instance of [`AppParams`](dmextension::AppParams).
pub fn get_hid_context(app_params: dmextension::AppParams) -> dmhid::Context {
    let ptr = unsafe { dmEngine::GetHIDContext(app_params.ptr) };
    dmhid::Context::new(ptr)
}
