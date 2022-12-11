use std::{
    ffi::{CStr, CString},
    mem::size_of,
};

pub type State = *mut dmsdk_ffi::lua_State;
pub type Function = extern "C" fn(l: State) -> i32;
pub type Reg = &'static [(&'static str, Function)];

/// Creates a new constant [`Reg`] with the name provided, to be used with [`register()`].
///
/// # Examples
/// ```
/// use dmsdk::*;
///
/// fn hello_world(l: lua::State) -> i32 {
///     unsafe { lua::push_string(l, "Hello, world!"); }
///
///     1
/// }
///
/// fn the_answer(l: lua::State) -> i32 {
///     unsafe { lua::push_integer(l, 42); }
///
///     1
/// }
///
/// // Equivalent to `const LUA_FUNCTIONS: lua::Reg = ...`
/// declare_functions!(
///     LUA_FUNCTIONS,
///     hello_world,
///     the_answer
/// );
/// ```
#[macro_export]
macro_rules! declare_functions {
    ($ident:ident, $($func:ident),*) => {
        paste! {
             const $ident: lua::Reg = &[$((stringify!($func), [<_wrapped_ $func>]), )*];

             $(
                #[no_mangle]
                extern "C" fn [<_wrapped_ $func>](l: lua::State) -> i32 {
                    $func(l)
                }
            )*
        }
    };
}

/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn push_userdata<T>(l: State, userdata: T) {
    let ptr = dmsdk_ffi::lua_newuserdata(
        l,
        size_of::<T>()
            .try_into()
            .expect("Failed to convert struct size"),
    ) as *mut T;
    ptr.write(userdata);
}

/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state and there is a valid instance of `T` at index `i` of the stack.
pub unsafe fn to_userdata<T>(l: State, i: i32) -> T {
    let ptr = dmsdk_ffi::lua_touserdata(l, i) as *mut T;
    ptr.read()
}

/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn push_integer(l: State, n: isize) {
    dmsdk_ffi::lua_pushinteger(l, n);
}

/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn push_string(l: State, s: &str) {
    let s = CString::new(s).unwrap();
    dmsdk_ffi::lua_pushstring(l, s.as_ptr());
}

/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn check_string(l: State, i: i32) -> String {
    let ptr = dmsdk_ffi::luaL_checklstring(l, i, std::ptr::null_mut());
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn check_bytes(l: State, i: i32) -> Vec<u8> {
    let mut length = 0;
    let ptr = dmsdk_ffi::luaL_checklstring(l, i, &mut length);

    // There's probably a better way to read X bytes from a ptr
    let mut vec = Vec::with_capacity(length as usize);
    for i in 0..length {
        vec.push(*ptr.add(i as usize) as u8);
    }
    vec
}

/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn get_top(l: State) -> i32 {
    dmsdk_ffi::lua_gettop(l)
}

/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn new_table(l: State) {
    dmsdk_ffi::lua_createtable(l, 0, 0);
}

/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn raw_set_i(l: State, index: i32, key: i32) {
    dmsdk_ffi::lua_rawseti(l, index, key);
}

/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn pop(l: State, n: i32) {
    dmsdk_ffi::lua_settop(l, -n - 1);
}

/// Taken from the [`mond` crate](https://github.com/blt/mond/blob/5028d86b4be0fdbba0a02e6e7802d7a64dfcda40/src/wrapper/state.rs#L1584).
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn register(l: State, lib_name: &str, functions: &[(&str, Function)]) {
    let lib_name = CString::new(lib_name).unwrap();

    let cstringed_fns: Vec<(CString, Function)> = functions
        .iter()
        .map(|&(name, func)| (CString::new(name).unwrap(), func))
        .collect();

    let mut lua_fns: Vec<dmsdk_ffi::luaL_Reg> = Vec::with_capacity(cstringed_fns.len() + 1);
    for &(ref name, func) in &cstringed_fns {
        lua_fns.push(dmsdk_ffi::luaL_Reg {
            name: name.as_ptr(),
            func: Some(func),
        });
    }

    lua_fns.push(dmsdk_ffi::luaL_Reg {
        name: std::ptr::null(),
        func: None,
    });

    dmsdk_ffi::luaL_register(l, lib_name.as_ptr(), lua_fns.as_ptr());
}

#[doc(hidden)]
pub unsafe fn __error(l: State, str: &str) -> ! {
    let s = CString::new(str).unwrap();
    dmsdk_ffi::luaL_error(l, s.as_ptr());
    panic!("luaL_error failed to return!")
}

#[doc(hidden)]
pub unsafe fn __push_fstring(l: State, str: &str) {
    let s = CString::new(str).unwrap();
    dmsdk_ffi::lua_pushfstring(l, s.as_ptr());
}

/// Raises an error with the given message.
///
/// The file name and line number will be added to the message, if available. This macro stops execution of the current function (returns `!`).
///
/// # Examples
/// ```
/// use dmsdk::*;
///
/// fn my_lua_fn(l: lua::State) -> i32 {
///     let number = 5;
///     if number < 11 {
///         lua::error!("Expected a number greater than 10, got {number}")
///     }
///
///     dmlog::warning!("This line shouldn't get printed!");
/// }
/// ```
#[macro_export]
macro_rules! __internal_lua_error {
    ($l:ident, $($arg:tt)*) => {
        lua::__error($l, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! __internal_push_fstring {
    ($l:ident, $($arg:tt)*) => {
        lua::__push_fstring($l, &format!($($arg)*));
    };
}

#[doc(inline)]
pub use crate::{__internal_lua_error as error, __internal_push_fstring as push_fstring};
