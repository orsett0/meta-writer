OUT := "target/wasm32-wasi/debug/meta-writer.wasm"

all: $(OUT) pkg/package.json

$(OUT): src/main.rs
	cargo build --release --target=wasm32-wasi
	cp $(OUT) pkg/

dev: pkg/package.json src/main.rs clean
	cargo build --target=wasm32-wasi

pkg/package.json: packager.sh Cargo.toml
	./packager.sh

clean:
	rm -rf target/ pkg/meta-writer.wasm