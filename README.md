download https://www.xquartz.org/

docker build -t pilot-os .

docker run --rm -it -v $(pwd):/PilotOS pilot-os

<!-- bare metal -->
rustup target add thumbv7em-none-eabihf

cargo build --target thumbv7em-none-eabihf

<!-- Alternate way to build -->
cargo rustc -- -C link-arg=-nostartfiles