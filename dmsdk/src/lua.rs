//! Wrappers for the Lua C API.

use std::{
    ffi::{CStr, CString},
    mem::size_of,
};

/// Mutable pointer to a [`lua_State`](crate::ffi::lua_State).
pub type State = *mut dmsdk_ffi::lua_State;
/// Alias for a Lua-compatible function.
pub type Function = extern "C" fn(l: State) -> i32;
/// Collection of Lua-compatible functions and their names.
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

/// Pushes any Rust object onto the stack as userdata.
///
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

/// Converts the data at `i` on the stack into an instance of type `T`.
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state and there is a valid instance of `T` at index `i` of the stack.
pub unsafe fn to_userdata<T>(l: State, i: i32) -> T {
    let ptr = dmsdk_ffi::lua_touserdata(l, i) as *mut T;
    ptr.read()
}

/// Pushes an [`isize`] onto the stack.
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn push_integer(l: State, n: isize) {
    dmsdk_ffi::lua_pushinteger(l, n);
}

/// Pushes a string slice onto the stack.
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn push_string(l: State, s: &str) {
    let s = CString::new(s).unwrap();
    dmsdk_ffi::lua_pushstring(l, s.as_ptr());
}

/// Checks if there is a Lua string at `i` and converts it into a [`String`].
///
/// This function uses [`CStr::from_ptr()`] and [`String::from_utf8_lossy()`], so the string will be cut short at nulls and any non-UTF8 sequences will be replaced with [`std::char::REPLACEMENT_CHARACTER`].
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn check_string(l: State, i: i32) -> String {
    let ptr = dmsdk_ffi::luaL_checklstring(l, i, std::ptr::null_mut());
    let cstr = CStr::from_ptr(ptr);
    String::from_utf8_lossy(cstr.to_bytes()).into_owned()
}

/// Checks if there is a Lua string at `i` and converts it into a [`Vec<u8>`].
///
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

/// Checks if there is a Lua number at `i` and converts it into an [`isize`].
///
/// The number will be rounded down to the nearest whole number.
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn check_int(l: State, i: i32) -> isize {
    dmsdk_ffi::luaL_checkinteger(l, i)
}

/// Checks if there is a Lua number at `i` and converts it into an [`f64`].
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn check_float(l: State, i: i32) -> f64 {
    dmsdk_ffi::luaL_checknumber(l, i)
}

/// Returns the number of elements in the stack.
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn get_top(l: State) -> i32 {
    dmsdk_ffi::lua_gettop(l)
}

/// Creates a new empty table and pushes it onto the stack.
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn new_table(l: State) {
    dmsdk_ffi::lua_createtable(l, 0, 0);
}

/// Sets the `n`th element of the table at `i` to the value on top of the stack.
///
/// The value at the top of the stack will be popped, and no metamethods will be invoked.
///
/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn raw_set_i(l: State, i: i32, n: i32) {
    dmsdk_ffi::lua_rawseti(l, i, n);
}

/// Pops `n` elements from the stack.
///
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
///         lua::error!(l, "Expected a number greater than 10, got {number}")
///     }
///
///     dmlog::warning!("This line shouldn't get printed!");
///
///     0
/// }
/// ```
#[macro_export]
macro_rules! __internal_lua_error {
    ($l:ident, $($arg:tt)*) => {
        lua::__error($l, &format!($($arg)*));
    };
}

/// Pushes a formatted string onto the stack.
///
/// # Examples
/// ```
/// use dmsdk::*;
///
/// fn greeting(l: lua::State) -> i32 {
///     unsafe {
///         let name = lua::check_string(l, 1);
///         lua::push_fstring!(l, "Hello, {name}!");
///     }
///
///     0
/// }
/// ```
#[macro_export]
macro_rules! __internal_push_fstring {
    ($l:ident, $($arg:tt)*) => {
        lua::__push_fstring($l, &format!($($arg)*));
    };
}

#[doc(inline)]
pub use crate::{__internal_lua_error as error, __internal_push_fstring as push_fstring};
