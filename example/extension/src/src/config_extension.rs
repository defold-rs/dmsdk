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
    default_value: &str,
) -> Option<String> {
    dmlog::info!("get_string({key}, \"{default_value}\")");

    if key == "my_section.my_value" {
        Some("It works!".to_owned())
    } else {
        None
    }
}
