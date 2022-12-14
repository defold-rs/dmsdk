#![allow(clippy::missing_safety_doc, clippy::not_unsafe_ptr_arg_deref)]

use dmsdk::*;

// LUA FUNCTIONS //
fn lua_function(l: lua::State) -> i32 {
    dmlog::info!("Hello from Rust!");

    unsafe {
        lua::push_integer(l, 123);
    }

    1
}

fn reverse(l: lua::State) -> i32 {
    let to_reverse = unsafe { lua::check_string(l, 1) };
    let reversed: String = to_reverse.chars().rev().collect();

    unsafe {
        lua::push_string(l, &reversed);
    }

    1
}

fn create_userdata(l: lua::State) -> i32 {
    let userdata = vec![1, 2, 3];
    unsafe {
        lua::push_userdata(l, userdata);
    }

    1
}

fn read_userdata(l: lua::State) -> i32 {
    let userdata: Vec<i32> = unsafe { lua::to_userdata(l, 1) };
    dmlog::info!("Userdata: {:?}", userdata);

    0
}

fn b64_encode(l: lua::State) -> i32 {
    unsafe {
        let plaintext = lua::check_string(l, 1);
        lua::push_string(l, &base64::encode(plaintext));
    };

    1
}

fn check_types(l: lua::State) -> i32 {
    unsafe {
        let int = lua::check_int(l, 1);
        let float = lua::check_float(l, 2);
        let string = lua::check_string(l, 3);
        let bytes = lua::check_bytes(l, 4);

        dmlog::info!(
            "int: {int}, float: {float}, string: \"{string}\", bytes: {:?}",
            bytes
        );
    }

    0
}

fn error(l: lua::State) -> i32 {
    unsafe {
        lua::error!(l, "An unexpected error occured!");
    }
}

declare_functions!(
    TEST,
    lua_function,
    reverse,
    create_userdata,
    read_userdata,
    b64_encode,
    check_types,
    error
);

// LIFECYCLE FUNCTIONS //
fn app_init(params: dmextension::AppParams) -> dmextension::Result {
    unsafe {
        let config = dmengine::get_config_file(params);

        let title = dmconfigfile::get_string(config, "project.title", "Untitled");
        let display_width = dmconfigfile::get_int(config, "display.width", 640);
        let gravity = dmconfigfile::get_float(config, "physics.gravity_y", -9.8);

        dmlog::info!("Display width is: {display_width}");
        dmlog::info!("Project title is: {title}");
        dmlog::info!("Gravity is: {gravity}");
    }

    let time = dmtime::get_time();
    dmlog::info!("Current time is: {time}");

    dmextension::Result::Ok
}

unsafe fn lua_init(l: lua::State) {
    let top = lua::get_top(l);

    lua::register(l, "rust", TEST);

    lua::pop(l, 1);

    assert_eq!(top, lua::get_top(l));
}

fn ext_init(params: dmextension::Params) -> dmextension::Result {
    unsafe {
        lua_init(params.l);
    }

    dmlog::info!("Registered Rust extension");

    dmextension::Result::Ok
}

fn ext_final(_params: dmextension::Params) -> dmextension::Result {
    dmextension::Result::Ok
}

declare_extension!(
    RUST,
    Some(app_init),
    None,
    Some(ext_init),
    Some(ext_final),
    None,
    None
);

// CONFIG FILE EXTENSION //
fn create(config: dmconfigfile::ConfigFile) {}

declare_configfile_extension!(RUST_CONFIG, Some(create), None, None, None, None);
