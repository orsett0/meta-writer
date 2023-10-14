OUT_DBG := "target/wasm32-wasi/debug/meta-writer.wasm"
OUT_REL := "target/wasm32-wasi/release/meta-writer.wasm"

all: $(OUT_REL) pkg/package.json
build: $(OUT_REL)
debug: $(OUT_DEV)

$(OUT_REL): src/main.rs
	cargo build --release --target=wasm32-wasi
	cp "target/wasm32-wasi/release/meta-writer.wasm" pkg/

$(OUT_DEV): pkg/package.json src/main.rs clean
	cargo build --target=wasm32-wasi
	cp -t pkg/ "target/wasm32-wasi/debug/meta-writer.wasm" "LICENSE" "README.md"

pkg/package.json: packager.sh Cargo.toml
	./packager.sh

clean:
	rm -rf target/ pkg/meta-writer.wasm