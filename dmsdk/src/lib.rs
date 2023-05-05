#![warn(missing_docs)]

//! # dmsdk
//!
//! Rust-friendly wrappers for interacting with the [Defold](https://defold.com) extension SDK.

pub mod dmconfigfile;
pub mod dmengine;
pub mod dmextension;
pub mod dmgameobject;
mod dmhash;
pub mod dmhid;
pub mod dmlog;
pub mod dmresource;
pub mod dmscript;
pub mod dmtime;
pub mod dmvmath;
pub mod dmwebserver;
pub mod lua;

pub use dmhash::*;

#[doc(hidden)]
pub use paste::paste;

#[doc(hidden)]
pub use ctor::ctor;

#[doc(hidden)]
pub use lazy_static::lazy_static;
