release:
  cargo build --release

build:
  cargo build --frozen

check:
  clear
  cargo c --frozen

format:
  clear
  cargo fmt
  cargo clippy --fix --allow-staged --allow-dirty

vendor:
  cargo vendor

clean:
  cargo clean

test:
  cargo test --frozen
