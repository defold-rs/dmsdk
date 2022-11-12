use std::{
    ffi::{CStr, CString},
    mem::size_of,
};

pub type State = *mut dmsdk_ffi::lua_State;
pub type Function = extern "C" fn(l: State) -> i32;
pub type Reg = &'static [(&'static str, Function)];

#[macro_export]
macro_rules! new_reg {
    ($($func:ident),*) => {
        &[$((stringify!($func), $func), )*]
    };
}

/// # Safety
///
/// This function is safe as long as `l` points to a valid Lua state.
pub unsafe fn push_userdata<T>(l: State, userdata: T) {
    let ptr = dmsdk_ffi::lua_newuserdata(l, size_of::<T>() as u64) as *mut T;
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
    let ptr = dmsdk_ffi::luaL_checklstring(l, i, std::ptr::null_mut());
    Vec::from(CStr::from_ptr(ptr).to_bytes())
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

/// Taken from the [`mond` crate](https://github.com/blt/mond/blob/5028d86b4be0fdbba0a02e6e7802d7a64dfcda40/src/wrapper/state.rs#L1584)
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
