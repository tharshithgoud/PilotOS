download https://www.xquartz.org/

docker build -t pilot-os .

docker run --rm -it -v $(pwd):/PilotOS pilot-os

<!-- bare metal -->
rustup target add --toolchain stable i686-unknown-linux-gnu

cargo build --target i686-pilot-os.json

<!-- Alternate way to build -->
cargo rustc -- -C link-arg=-nostartfiles

<!-- Setup Rust Nightly -->
rustup toolchain install nightly

rustup override set nightly

cargo +nightly build
