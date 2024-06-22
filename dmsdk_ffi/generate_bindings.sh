bindgen sdk.h --enable-cxx-namespaces --allowlist-file ".*dmsdk.*" -- -I. -x c++ > src/bindings.rs
# bindgen sdk.h --enable-cxx-namespaces --allowlist-item "dm.*" --allowlist-item "lua.*" --allowlist-function "Log.*" -- -I. -x c++ > src/bindings.rs
# bindgen dmsdk/resource/resource.h --opaque-type ".*Desc" --enable-cxx-namespaces --allowlist-item "dm.*" --allowlist-item "lua.*" -- -I. -x c++ > src/resource.rs
# bindgen dmsdk/gameobject/component.h --enable-cxx-namespaces --allowlist-item "dm.*" --allowlist-item "lua.*" -- -I. -x c++ > src/component.rs