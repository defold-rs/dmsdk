use dmsdk::*;

pub fn create(_config: dmconfigfile::ConfigFile) {
    dmlog::info!("create()");
}

pub fn destroy(_config: dmconfigfile::ConfigFile) {
    dmlog::info!("destroy()");
}

pub fn get_string(
    _config: dmconfigfile::ConfigFile,
    key: &str,
    _default_value: &str,
) -> Option<String> {
    if key == "my_section.my_value" {
        Some("It works!".to_owned())
    } else {
        None
    }
}
