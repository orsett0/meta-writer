'use strict';
const { readFile } = require('node:fs/promises');
const { realpathSync } = require('node:fs'); 
const { WASI } = require('wasi');
const { argv, env } = require('node:process');
const { join, dirname, basename } = require('node:path');

const WASMpath = realpathSync('../../target/wasm32-wasi/debug/lofty-wasm.wasm');

var file_rp = realpathSync(argv.at(-1));
var directory = dirname(file_rp);
var filename = basename(file_rp);

const wasi = new WASI({
  version: 'preview1',
  args: argv.slice(1, argv.length - 1).concat([join('/sandbox', filename)]),
  env,
  preopens: {
    '/sandbox': directory,
  },
});

(async () => {
  const wasm = await WebAssembly.compile(
    await readFile(WASMpath),
  );
  const instance = await WebAssembly.instantiate(wasm, wasi.getImportObject());

  wasi.start(instance);
})();
