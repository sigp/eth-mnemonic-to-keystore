CARGO_TOML = "emtk/Cargo.toml"

install:
	cargo install --path emtk

build-x86_64:
	cross build --manifest-path $(CARGO_TOML) --target x86_64-unknown-linux-gnu --locked
build-aarch64:
	cross build --manifest-path $(CARGO_TOML) --target aarch64-unknown-linux-gnu --locked