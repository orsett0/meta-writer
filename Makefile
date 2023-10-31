OUT_DEV := "target/wasm32-wasi/debug/meta-writer.wasm"
OUT_REL := "target/wasm32-wasi/release/meta-writer.wasm"

all: $(OUT_REL) pkg/package.json
build: $(OUT_REL)
debug: $(OUT_DEV)

$(OUT_REL): src/main.rs
	cargo build --release --target=wasm32-wasi
	cp -t pkg/ "target/wasm32-wasi/release/meta-writer.wasm" "LICENSE" "README.md"

$(OUT_DEV): pkg/package.json src/main.rs
	cargo build --target=wasm32-wasi
	cp -t pkg/ "target/wasm32-wasi/debug/meta-writer.wasm" "LICENSE" "README.md"

pkg/package.json: tools/packager.js Cargo.toml
	node tools/packager.js

clean:
	rm -rf target/ pkg/meta-writer.wasm