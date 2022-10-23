#![allow(clippy::missing_safety_doc, clippy::not_unsafe_ptr_arg_deref)]

use std::{
    ffi::{c_void, CStr},
    ptr,
};

use dmsdk::*;

extern "C" {
    //fn pos(instance: dmsdk::ffi::dmGameObject::HInstance) -> dmsdk::ffi::dmVMath::Point3;
}

// LUA FUNCTIONS //
#[no_mangle]
pub extern "C" fn lua_function(l: lua::State) -> i32 {
    dmlog::info("RUST", "Hello from Rust!");

    unsafe {
        lua::push_integer(l, 123);
        let go = *dmscript::check_go_instance(l);
        println!("{:#?}", go);
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
pub extern "C" fn b64_encode(l: lua::State) -> i32 {
    unsafe {
        let plaintext = lua::check_string(l, 1);
        lua::push_string(l, &plaintext);
    };

    1
}

#[no_mangle]
pub extern "C" fn get_position(l: lua::State) -> i32 {
    let instance = unsafe { dmscript::check_go_instance(l) };

    //unsafe {
    /*dmgameobject::set_position(
        instance,
        dmvmath::Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
    );*/
    //}
    unsafe {
        println!("{:?}", dmgameobject::get_identifier(instance));
        //println!("{:?}", pos(instance));
    }

    //let position = unsafe { dmgameobject::get_position(instance) };
    //let rotation = unsafe { dmgameobject::get_rotation(instance) };
    /*let id = dmgameobject::get_identifier(&instance);
    let scale = dmgameobject::get_scale(&instance);
    println!("{:?} {:?} {:?}", position, rotation, scale);*/

    unsafe {
        dmscript::push_vector3(
            l,
            dmvmath::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        );
    }

    1
}

// #[no_mangle]
// pub extern "C" fn play_sound(l: lua::State) -> i32 {
//     let bytes = unsafe { lua::check_bytes(l, 1) };
//     let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();
//     let sound_data =
//         StaticSoundData::from_cursor(Cursor::new(bytes), StaticSoundSettings::default()).unwrap();

//     manager.play(sound_data).unwrap();

//     0
// }

const LUA_FUNCTIONS: lua::Reg = &[
    ("reverse", reverse),
    ("lua_function", lua_function),
    ("b64_encode", b64_encode),
    ("get_position", get_position), //    ("play_sound", play_sound),
];

// LIFECYCLE FUNCTIONS //
#[no_mangle]
pub unsafe extern "C" fn app_init(params: dmextension::AppParams) -> i32 {
    let config = dmengine::get_config_file(params);

    let title = dmconfigfile::get_string(config, "project.title", "Untitled");
    let display_width = dmconfigfile::get_int(config, "display.width", 640);
    let gravity = dmconfigfile::get_float(config, "physics.gravity_y", -9.8);

    dmlog::info("RUST", &format!("Display width is: {display_width}"));
    dmlog::info("RUST", &format!("Project title is: {title}"));
    dmlog::info("RUST", &format!("Gravity is: {gravity}"));

    dmlog::info("RUST", &format!("Current time is: {}", dmtime::get_time()));

    dmextension::RESULT_OK
}

#[no_mangle]
pub unsafe extern "C" fn lua_init(l: lua::State) {
    let top = lua::get_top(l);

    lua::register(l, "rust", LUA_FUNCTIONS);

    lua::pop(l, 1);

    assert_eq!(top, lua::get_top(l));
}

#[no_mangle]
pub unsafe extern "C" fn ext_init(params: dmextension::Params) -> i32 {
    lua_init((*params).m_L);
    dmlog::info("RUST", "Registered Rust Extension");

    let json = "{\"foo\": \"bar\", \"cool_number\": 1234}";
    match dmjson::parse(json) {
        dmjson::Result::Ok(document) => {
            dmlog::info("RUST", &format!("dmjson::parse() -> {:#?}", document))
        }
        dmjson::Result::Err(err) => {
            println!("Error parsing JSON: {:?}", err)
        }
    }

    dmextension::RESULT_OK
}

#[no_mangle]
pub extern "C" fn ext_final(_params: dmextension::Params) -> i32 {
    dmextension::RESULT_OK
}

declare_extension!(RUST, ext_init, ext_final, Some(app_init), None, None, None);

// RESOURCE TYPE CREATION //
pub struct Something {
    field: [u8; 4],
}

static mut SOMETHING_RESOURCE: Something = Something { field: [0; 4] };

#[no_mangle]
unsafe extern "C" fn something_create(params: *const dmresource::ResourceCreateParams) -> i32 {
    let mut resource = *(*params).m_Resource;
    resource.m_Resource = &mut SOMETHING_RESOURCE as *mut _ as *mut c_void;
    resource.m_ResourceSize = 4;
    println!("{:?}", SOMETHING_RESOURCE.field);

    0
}

#[no_mangle]
unsafe extern "C" fn something_destroy(_params: *const dmresource::ResourceDestroyParams) -> i32 {
    0
}

#[no_mangle]
unsafe extern "C" fn something_recreate(_params: *const dmresource::ResourceRecreateParams) -> i32 {
    0
}

#[no_mangle]
unsafe extern "C" fn something_register(
    params: *mut dmresource::ResourceTypeRegisterContext,
) -> i32 {
    dmresource::register_type(
        (*params).m_Factory,
        (*params).m_Name,
        ptr::null_mut(),
        None,
        Some(something_create),
        None,
        Some(something_destroy),
        Some(something_recreate),
    );

    dmlog::info(
        "RUST",
        &format!(
            "Registered type {:?}",
            CStr::from_ptr((*params).m_Name).to_str().unwrap()
        ),
    );

    0
}

register_resource_type!(RUSTC, "smthc", something_register, None);
