cd extension/rust
RUSTC_WRAPPER=sccache cargo build --release
mkdir -p ../lib/$1
cp ./target/release/libextension.a ../lib/$1/librust.a