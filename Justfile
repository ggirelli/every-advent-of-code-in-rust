ci: && fmt check test done

fmt:
    cargo fmt

test:
    cargo test

check:
    cargo check
    cargo clippy

build:
    cargo build

done:
    @echo "✨✨ Done ✨✨"

run year day part input:
    @RUST_LOG=DEBUG cargo run -- -y {{year}} -d {{day}} -p {{part}} -i {{input}}
    @just done