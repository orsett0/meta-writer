# Step by step guid to build a WebAssemply thingy with rust.

This project is basically a write-only wrapper for lofty-ls, compiled to WASM.

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

This software is licensed under the Apache 2.0 license - Copyright 2023 Alessio Orsini <alessiorsini.ao@proton.me>

The file `samples/service-login.oga` and its re-encodings are licensed under the GPLv2, by The Pidgin developers. `service-login.oga` is part of the package [https://cgit.freedesktop.org/sound-theme-freedesktop/](sound-theme-freedesktop).