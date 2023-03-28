extern crate bindgen;

use glob::glob;
use std::{collections::HashSet, env, path::PathBuf};

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn _defold_allowlist(mut builder: bindgen::Builder) -> bindgen::Builder {
    for entry in glob("dmsdk/**/*.h").unwrap().filter_map(Result::ok) {
        //println!("{:?}", entry.into_os_string().into_string().unwrap());
        builder = builder.allowlist_file(entry.into_os_string().into_string().unwrap());
    }

    builder
}

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    //println!("cargo:rustc-link-search=/path/to/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    //println!("cargo:rerun-if-changed=wrapper.h");

    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),
            "FP_INT_UPWARD".into(),
            "FP_INT_DOWNWARD".into(),
            "FP_INT_TOWARDZERO".into(),
            "FP_INT_TONEARESTFROMZERO".into(),
            "FP_INT_TONEAREST".into(),
        ]
        .into_iter()
        .collect(),
    );

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default() //defold_allowlist(bindgen::Builder::default())
        .enable_cxx_namespaces()
        // Set the include path and force C++ mode
        .clang_args(vec!["-I.", "-x", "c++"].iter())
        // The input header we would like to generate
        // bindings for.
        .header("dmsdk/sdk.h")
        .header("dmsdk/resource/resource.h")
        .header("dmsdk/gameobject/component.h")
        //.blocklist_file("graphics_native.h")
        //.blocklist_file("dmsdk/graphics/glfw/glfw.h")
        .blocklist_item("std")
        .blocklist_item("__gnu.*")
        .allowlist_function("Log.*")
        .allowlist_function("dm.*")
        .allowlist_function("lua.*")
        .allowlist_type("dm.*")
        .allowlist_type("lua.*")
        .opaque_type(".*Desc")
        .opaque_type(".*Transform.*")
        //.allowlist_recursively(false)
        .time_phases(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(ignored_macros))
        .rustfmt_bindings(true);

    let bindings = builder
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
