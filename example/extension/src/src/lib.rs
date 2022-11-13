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

declare_functions!(
    TEST,
    lua_function,
    reverse,
    create_userdata,
    read_userdata,
    b64_encode
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

    let json = "{\"foo\": \"bar\", \"cool_number\": 1234}";
    match dmjson::parse(json) {
        dmjson::Result::Ok(document) => {
            dmlog::info!("dmjson::parse() -> {:#?}", document);
        }
        dmjson::Result::Err(err) => {
            println!("Error parsing JSON: {:?}", err)
        }
    }

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
