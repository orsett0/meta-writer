OUT := "target/wasm32-wasi/debug/lofty-wasm.wasm"

all: $(OUT) pkg/package.json

$(OUT): src/main.rs
	cargo build --target=wasm32-wasi
	cp $(OUT) pkg/

pkg/package.json: packager.sh Cargo.toml
	./packager.sh
