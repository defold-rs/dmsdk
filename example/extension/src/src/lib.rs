use std::ffi::c_void;

use dmsdk::{ffi::dmResource, *};

// LUA FUNCTIONS //
fn lua_function(l: lua::State) -> i32 {
    dmlog::info!("Hello from Rust!");

    lua::push_integer(l, 123);

    1
}

fn reverse(l: lua::State) -> i32 {
    let to_reverse = lua::check_string(l, 1);
    let reversed: String = to_reverse.chars().rev().collect();

    lua::push_string(l, &reversed);

    1
}

fn create_userdata(l: lua::State) -> i32 {
    let userdata = vec![1, 2, 3];
    lua::push_userdata(l, userdata);

    1
}

fn read_userdata(l: lua::State) -> i32 {
    let userdata: Vec<i32> = unsafe { lua::to_userdata(l, 1) };
    dmlog::info!("Userdata: {:?}", userdata);

    0
}

fn b64_encode(l: lua::State) -> i32 {
    let plaintext = lua::check_string(l, 1);
    lua::push_string(l, &base64::encode(plaintext));

    1
}

fn check_types(l: lua::State) -> i32 {
    let int = lua::check_int(l, 1);
    let float = lua::check_float(l, 2);
    let string = lua::check_string(l, 3);
    let bytes = lua::check_bytes(l, 4);
    let boolean = lua::to_bool(l, 5);

    dmlog::info!(
        "int: {int}, float: {float}, string: \"{string}\", bytes: {:?}, bool: {boolean}",
        bytes
    );

    0
}

fn error(l: lua::State) -> i32 {
    lua::error!(l, "An expected error occured!");
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

fn lua_init(l: lua::State) {
    let top = lua::get_top(l);

    lua::register(l, "rust", TEST);

    lua::pop(l, 1);

    assert_eq!(top, lua::get_top(l));
}

fn ext_init(params: dmextension::Params) -> dmextension::Result {
    lua_init(params.l);

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

mod config_extension;

declare_configfile_extension!(
    RUST_CONFIG,
    Some(config_extension::create),
    Some(config_extension::destroy),
    Some(config_extension::get_string),
    None,
    None
);

mod custom_component;
declare_component_type!(
    MY_COMPONENT,
    "testc",
    custom_component::create_type,
    Some(custom_component::destroy_type)
);

#[no_mangle]
unsafe extern "C" fn resource_create(params: *const dmResource::ResourceCreateParams) -> i32 {
    0
}

#[no_mangle]
unsafe extern "C" fn resource_destroy(params: *const dmResource::ResourceDestroyParams) -> i32 {
    0
}

#[no_mangle]
unsafe extern "C" fn resource_type_register(
    ctx: *mut dmResource::ResourceTypeRegisterContext,
) -> i32 {
    let ctx = *ctx;
    dmresource::register_type(
        ctx.m_Factory,
        ctx.m_Name,
        &mut () as *mut _ as *mut c_void,
        None,
        Some(resource_create),
        None,
        Some(resource_destroy),
        None,
    )
}

declare_resource_type!(MY_RESOURCE, "testc", resource_type_register, None);
