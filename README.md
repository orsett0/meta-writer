# Meta-Writer 

Tool to add metadata to media files using [lofty-rs](https://github.com/Serial-ATA/lofty-rs/), compiled to WASM. It tries to add metadata of the types available in the enum `lofty::ItemKey`, and then tries to add the unmatched values as an [Ilst](https://developer.apple.com/documentation/quicktime-file-format/metadata_item_list_atom) tag, if the file is an `mp4`.

This project should basically be a write-only wrapper for lofty-ls, compiled to WASM. It is currently only tested with mp3 and mp4 formats.

I'm using a `Makefile` because I'm too lazy to learn [cargo-make](https://crates.io/crates/cargo-make), and also it seems like it does not support a file as dependency (altough I didn't look much into it).

## Building steps

```
rustup target add wasm32-wasi
make
```

## Sources:

- [https://rustwasm.github.io/docs/book/game-of-life/hello-world.html](rustwasm.github.io)
- [https://nodejs.dev/en/learn/nodejs-with-webassembly/](nodejs.web)
- [https://github.com/kubkon/rust-wasi-tutorial](rust-wasi-tutorial) (**The only true source**)

## License

This software and all the file in this repository, unless otherwise specified, is licensed under the Apache 2.0 license - Copyright 2023 Alessio Orsini <alessiorsini.ao@proton.me>. The full text of the license is located in the file [LICENSE](./LICENSE)
