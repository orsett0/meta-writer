'use strict';

import { readFile } from 'node:fs/promises';
import { WASI } from 'wasi';
import { argv, env } from 'node:process';
import { join, dirname, basename } from 'node:path';
import { realpathSync } from 'node:fs';

// There's probably a better way to reference a file in the module directory
const WASMpath = 'node_modules/lofty-wasm/lofty-wasm.wasm';

export async function lofty(metadata, file) {
  var directory = realpathSync(dirname(file));
  var filename = basename(file);

  const wasi = new WASI({
    version: 'preview1',
    args: [
      argv[0],
      JSON.stringify(metadata),
      join('/sandbox', filename)
    ],
    env,
    preopens: {
      '/sandbox': directory,
    },
  });

  const wasm = await WebAssembly.compile(
    await readFile(WASMpath),
  );
  const instance = await WebAssembly.instantiate(wasm, wasi.getImportObject());
  wasi.start(instance);
}