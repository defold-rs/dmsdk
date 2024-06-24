for target in x86_64-unknown-linux-gnu x86_64-pc-windows-msvc x86_64-apple-darwin aarch64-apple-darwin; do
    bindgen sdk.h --enable-cxx-namespaces --allowlist-file ".*dmsdk.*" -- -I. -x c++ $1 > src/bindings-$target.rs
done