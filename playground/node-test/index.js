'use strict';
const { readFile } = require('node:fs/promises');
const { WASI } = require('wasi');
const { argv, env } = require('node:process');
const { join } = require('node:path');

const wasi = new WASI({
  version: 'preview1',
  args: argv.slice(1, argv.length),
  env,
  preopens: {
    '/sandbox': '.',
  },
});

(async () => {
  const wasm = await WebAssembly.compile(
    await readFile(join(__dirname, '../../target/wasm32-wasi/debug/lofty-wasm.wasm')),
  );
  const instance = await WebAssembly.instantiate(wasm, wasi.getImportObject());

  wasi.start(instance);
})();
