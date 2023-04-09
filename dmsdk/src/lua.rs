//! Wrappers for the Lua C API.

use std::{
    ffi::{CStr, CString},
    mem::size_of,
};

/// Mutable pointer to a [`lua_State`](crate::ffi::lua_State).
pub type StatePtr = *mut dmsdk_ffi::lua_State;
/// Alias for a Lua-compatible function.
pub type Function = extern "C" fn(l: StatePtr) -> i32;
/// Collection of Lua-compatible functions and their names.
pub type Reg = &'static [(&'static str, Function)];

/// Wrapper around a raw [`StatePtr`].
#[derive(Clone, Copy)]
pub struct State {
    ptr: StatePtr,
}

impl State {
    /// # Safety
    ///
    /// This function is safe as long as `ptr` points to a valid Lua state.
    pub unsafe fn new(ptr: StatePtr) -> Self {
        Self { ptr }
    }

    /// Returns the inner pointer.
    pub fn ptr(&self) -> StatePtr {
        self.ptr
    }
}

/// Creates a new constant [`Reg`] with the name provided, to be used with [`register()`].
///
/// # Examples
/// ```
/// use dmsdk::*;
///
/// fn hello_world(l: lua::State) -> i32 {
///     lua::push_string(l, "Hello, world!");
///
///     1
/// }
///
/// fn the_answer(l: lua::State) -> i32 {
///     lua::push_integer(l, 42);
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
                extern "C" fn [<_wrapped_ $func>](l: lua::StatePtr) -> i32 {
                    unsafe {
                        $func(lua::State::new(l))
                    }
                }
            )*
        }
    };
}

/// Pushes any Rust object onto the stack as userdata.
pub fn push_userdata<T>(l: State, userdata: T) {
    unsafe {
        let ptr = dmsdk_ffi::lua_newuserdata(l.ptr, size_of::<T>()) as *mut T;
        ptr.write(userdata);
    }
}

/// Converts the data at `i` on the stack into an instance of type `T`.
///
/// # Safety
///
/// This function is safe as long as there is a valid instance of `T` at index `i` of the stack.
pub unsafe fn to_userdata<T>(l: State, i: i32) -> T {
    let ptr = dmsdk_ffi::lua_touserdata(l.ptr, i) as *mut T;
    ptr.read()
}

/// Pushes an [`isize`] onto the stack.
pub fn push_integer(l: State, n: isize) {
    unsafe {
        dmsdk_ffi::lua_pushinteger(l.ptr, n);
    }
}

/// Pushes a string slice onto the stack.
pub fn push_string(l: State, s: &str) {
    let s = CString::new(s).unwrap();
    unsafe {
        dmsdk_ffi::lua_pushstring(l.ptr, s.as_ptr());
    }
}

/// Checks if there is a Lua string at `i` and converts it into a [`String`].
///
/// This function uses [`CStr::from_ptr()`] and [`String::from_utf8_lossy()`],
/// so the string will be cut short at nulls and any non-UTF8 sequences will be replaced with [`std::char::REPLACEMENT_CHARACTER`].
pub fn check_string(l: State, i: i32) -> String {
    unsafe {
        let ptr = dmsdk_ffi::luaL_checklstring(l.ptr, i, std::ptr::null_mut());
        let cstr = CStr::from_ptr(ptr);
        String::from_utf8_lossy(cstr.to_bytes()).into_owned()
    }
}

/// Checks if there is a Lua string at `i` and converts it into a [`Vec<u8>`].
pub fn check_bytes(l: State, i: i32) -> Vec<u8> {
    let mut length = 0;
    let ptr = unsafe { dmsdk_ffi::luaL_checklstring(l.ptr, i, &mut length) };

    // There's probably a better way to read X bytes from a ptr
    let mut vec = Vec::with_capacity(length);
    for i in 0..length {
        let byte = unsafe { *ptr.add(i) as u8 };
        vec.push(byte);
    }
    vec
}

/// Checks if there is a Lua number at `i` and converts it into an [`isize`].
///
/// The number will be rounded down to the nearest whole number.
pub fn check_int(l: State, i: i32) -> isize {
    unsafe { dmsdk_ffi::luaL_checkinteger(l.ptr, i) }
}

/// Checks if there is a Lua number at `i` and converts it into an [`f64`].
pub fn check_float(l: State, i: i32) -> f64 {
    unsafe { dmsdk_ffi::luaL_checknumber(l.ptr, i) }
}

/// Returns `true` if the value at `i` is not `false` or `nil`.
pub fn to_bool(l: State, i: i32) -> bool {
    unsafe { dmsdk_ffi::lua_toboolean(l.ptr, i) > 0 }
}

/// Returns the number of elements in the stack.
pub fn get_top(l: State) -> i32 {
    unsafe { dmsdk_ffi::lua_gettop(l.ptr) }
}

/// Creates a new empty table and pushes it onto the stack.
pub fn new_table(l: State) {
    unsafe {
        dmsdk_ffi::lua_createtable(l.ptr, 0, 0);
    }
}

/// Sets the `n`th element of the table at `i` to the value on top of the stack.
///
/// The value at the top of the stack will be popped, and no metamethods will be invoked.
pub fn raw_set_i(l: State, i: i32, n: i32) {
    unsafe {
        dmsdk_ffi::lua_rawseti(l.ptr, i, n);
    }
}

/// Pops `n` elements from the stack.
pub fn pop(l: State, n: i32) {
    unsafe {
        dmsdk_ffi::lua_settop(l.ptr, -n - 1);
    }
}

/// Taken from the [`mond` crate](https://github.com/blt/mond/blob/5028d86b4be0fdbba0a02e6e7802d7a64dfcda40/src/wrapper/state.rs#L1584).
pub fn register(l: State, lib_name: &str, functions: &[(&str, Function)]) {
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

    unsafe {
        dmsdk_ffi::luaL_register(l.ptr, lib_name.as_ptr(), lua_fns.as_ptr());
    }
}

#[doc(hidden)]
pub fn __error(l: State, str: &str) -> ! {
    let s = CString::new(str).unwrap();
    unsafe {
        dmsdk_ffi::luaL_error(l.ptr, s.as_ptr());
    }
    panic!("luaL_error failed to return!")
}

#[doc(hidden)]
pub fn __push_fstring(l: State, str: &str) {
    let s = CString::new(str).unwrap();
    unsafe {
        dmsdk_ffi::lua_pushfstring(l.ptr, s.as_ptr());
    }
}

/// Raises an error with the given message.
///
/// The file name and line number will be added to the message, if available.
/// This macro stops execution of the current function (returns `!`).
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
///     let name = lua::check_string(l, 1);
///     lua::push_fstring!(l, "Hello, {name}!");
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
pub use crate::{
    __internal_lua_error as error, __internal_push_fstring as push_fstring, declare_functions,
};
