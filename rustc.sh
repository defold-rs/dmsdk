cd extension/rust
RUSTC_WRAPPER=sccache cargo build --release
cp ./target/release/libextension.a ../lib/$1/librust.a
