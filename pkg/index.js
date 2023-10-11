/*
Copyright 2023 Alessio Orsini <alessiorsini.ao@proton.me>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

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