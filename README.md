![defold-rs](assets/defold-rs.svg)

# defold-rs
![Crates.io](https://img.shields.io/crates/v/dmsdk)

Tools for creating [Defold](https://defold.com/) native extensions in Rust.

# Quick Start
You will need:
- [Defold](https://defold.com/download/)
- [Docker](https://www.docker.com/)
- [The template project](https://github.com/JustAPotota/defold-rs-template)
- [The extension build server](https://github.com/JustAPotota/defold-rs-extender)

Follow the instructions in the build server's README to get it built and running. Once it's up, open the template project in Defold and open `/myextension/src` in a Rust-compatible IDE. To tell Defold to use your build server, select `File > Preferences > Extensions` and set `Build Server` to `http://localhost:9000`. Once that's done, you can just press `Ctrl+B` in Defold to begin compiling your project! The first build will take some time while it downloads files from Defold's servers and compiles the crate from scratch.

# What is Defold?
[Defold](https://defold.com/) is a free and open-source game engine focused on making fast, lightweight games for desktop, web, mobile, and console with zero setup. Almost all game logic is written in [Lua](https://en.wikipedia.org/wiki/Lua_(programming_language)) scripts, but the engine also provides a way to extend its functionality with C++ using [native extensions](https://defold.com/manuals/extensions/).

# What are native extensions?
[Native extensions](https://defold.com/manuals/extensions/) allow you to extend Defold's functionality using C++ or any platform-specific language. You can write extensions in JavaScript for web games, Java for Android, or Objective-C for macOS/iOS. Defold's extension SDK (dmSDK for short) grants you much deeper control of the engine than standard Lua scripts and is great for performance-sensitive processing.

# So what's `defold-rs` then?
It's two main pieces that come together to let you write native extensions in Rust:

- [`defold-rs`](https://github.com/JustAPotoa/defold-rs) - This repository is home to the [`dmsdk`](https://crates.io/crates/dmsdk) and [`dmsdk_ffi`](https://crates.io/crates/dmsdk_ffi) crates. In short, `dmsdk_ffi` contains auto-generated bindings to the dmSDK and `dmsdk` wraps those unsafe bindings into a nice Rust-y package. See the crates' READMEs for more details.
- [`defold-rs-extender`](https://github.com/JustAPotoa/defold-rs-extender) - A fork of Defold's extension build server with added Rust support. You'll need to run one of these yourself in order to use this project.