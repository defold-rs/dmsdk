use base64::{engine::general_purpose::STANDARD as b64, Engine};
use dmsdk::*;

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
    lua::push_string(l, &b64.encode(plaintext));

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
    LUA_FUNCTIONS,
    lua_function,
    reverse,
    create_userdata,
    read_userdata,
    b64_encode,
    check_types,
    error
);

fn lua_init(l: lua::State) {
    let top = lua::get_top(l);

    lua::register(l, "rust", LUA_FUNCTIONS);
    lua::pop(l, 1);

    assert_eq!(top, lua::get_top(l));
}

#[derive(Default)]
struct RustExt {
    timer: i32,
    pressed: bool,
    // There's no such thing as a "default" context,
    // so we wrap it in an Option
    hid_context: Option<dmhid::Context>,
}

impl dmextension::Extension for RustExt {
    fn app_init(&mut self, params: dmextension::AppParams) -> dmextension::Result {
        dmlog::info!("Cool!");

        self.hid_context = Some(dmengine::get_hid_context(params));

        dmextension::Result::Ok
    }

    fn ext_init(&mut self, params: dmextension::Params) -> dmextension::Result {
        lua_init(params.l);
        dmlog::info!("Registered 'rust' module");

        dmextension::Result::Ok
    }

    fn on_update(&mut self, _params: dmextension::Params) -> dmextension::Result {
        self.timer += 1;

        // Toggle pressed every 60 updates
        if self.timer % 60 == 0 {
            self.pressed = !self.pressed;
        }

        if let Some(hid) = &self.hid_context {
            if let Some(mouse) = hid.get_mouse(0) {
                mouse.set_button(dmhid::MouseButton::Left, self.pressed);
            }
        }

        dmextension::Result::Ok
    }
}

declare_extension!(RustExt);

mod config_extension;

declare_configfile_extension!(
    RUST_CONFIG,
    Some(config_extension::create),
    Some(config_extension::destroy),
    Some(config_extension::get_string),
    None,
    None
);
