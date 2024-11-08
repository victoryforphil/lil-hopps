#cross build --release --bins --target arm-unknown-linux-gnueabihf
rustup target add arm-unknown-linux-gnueabihf
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target arm-unknown-linux-gnueabihf --bin quad_idle
