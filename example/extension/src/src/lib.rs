#![allow(clippy::missing_safety_doc, clippy::not_unsafe_ptr_arg_deref)]

use dmsdk::*;

// LUA FUNCTIONS //
#[no_mangle]
pub extern "C" fn lua_function(l: lua::State) -> i32 {
    dmsdk::info!("Hello from Rust!");

    unsafe {
        lua::push_integer(l, 123);
    }

    1
}

#[no_mangle]
pub extern "C" fn reverse(l: lua::State) -> i32 {
    let to_reverse = unsafe { lua::check_string(l, 1) };
    let reversed: String = to_reverse.chars().rev().collect();

    unsafe {
        lua::push_string(l, &reversed);
    }

    1
}

#[no_mangle]
extern "C" fn create_userdata(l: lua::State) -> i32 {
    let userdata = vec![1, 2, 3];
    unsafe {
        lua::push_userdata(l, userdata);
    }

    1
}

#[no_mangle]
extern "C" fn read_userdata(l: lua::State) -> i32 {
    let userdata: Vec<i32> = unsafe { lua::to_userdata(l, 1) };
    dmsdk::info!("Userdata: {:?}", userdata);

    0
}

#[no_mangle]
pub extern "C" fn b64_encode(l: lua::State) -> i32 {
    unsafe {
        let plaintext = lua::check_string(l, 1);
        lua::push_string(l, &base64::encode(plaintext));
    };

    1
}

const LUA_FUNCTIONS: lua::Reg = new_reg!(
    reverse,
    lua_function,
    b64_encode,
    create_userdata,
    read_userdata
);

// LIFECYCLE FUNCTIONS //
fn app_init(params: dmextension::AppParams) -> dmextension::Result {
    unsafe {
        let config = dmengine::get_config_file(params);

        let title = dmconfigfile::get_string(config, "project.title", "Untitled");
        let display_width = dmconfigfile::get_int(config, "display.width", 640);
        let gravity = dmconfigfile::get_float(config, "physics.gravity_y", -9.8);

        dmsdk::info!("Display width is: {display_width}");
        dmsdk::info!("Project title is: {title}");
        dmsdk::info!("Gravity is: {gravity}");
    }

    let time = dmtime::get_time();
    dmsdk::info!("Current time is: {time}");

    dmextension::Result::Ok
}

unsafe fn lua_init(l: lua::State) {
    let top = lua::get_top(l);

    lua::register(l, "rust", LUA_FUNCTIONS);

    lua::pop(l, 1);

    assert_eq!(top, lua::get_top(l));
}

fn ext_init(params: dmextension::Params) -> dmextension::Result {
    unsafe {
        lua_init(params.l);
    }

    dmsdk::info!("Registered Rust extension");

    let json = "{\"foo\": \"bar\", \"cool_number\": 1234}";
    match dmjson::parse(json) {
        dmjson::Result::Ok(document) => {
            dmsdk::info!("dmjson::parse() -> {:#?}", document);
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
