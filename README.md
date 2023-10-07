# Step by step guid to build a WebAssemply thingy with rust.

## Building steps

```
# cargo init
rustup target add wasm32-wasi
cargo build --target=wasm32-wasi
```

## Sources:

- [https://rustwasm.github.io/docs/book/game-of-life/hello-world.html](rustwasm.github.io)
- [https://nodejs.dev/en/learn/nodejs-with-webassembly/](nodejs.web)

### The only true source right here:

- [https://github.com/kubkon/rust-wasi-tutorial](rust-wasi-tutorial)

## License

Unless otherwise specified, this project will one day be licensed under the GPLv3.

The file service-login.oga is licensed under the GPLv2, by The Pidgin developers, and it's part of the package [https://cgit.freedesktop.org/sound-theme-freedesktop/](sound-theme-freedesktop).