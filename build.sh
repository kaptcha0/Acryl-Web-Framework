echo "installing default toolchain"
rustup default stable
rustup component add rustc
rustup component add cargo

clear

echo "installing cargo-watch"
cargo install cargo-watch
clear

echo "building"
cargo build