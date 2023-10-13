/*
meta-writerCopyright 2023 Alessio Orsini <alessiorsini.ao@proton.me>
SPDX-License-Identifier: Apache-2.0

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
const WASMpath = 'node_modules/meta-writer/meta-writer.wasm';

export async function lofty(metadata, file) {
  var preopens = { '/sandbox': realpathSync(dirname(file)) };

  if (metadata['FrontCover'] !== undefined) {
    preopens['/sandbox/cover'] = realpathSync(dirname(metadata['FrontCover']));
    metadata['FrontCover'] = join('/sandbox/cover', basename(metadata['FrontCover']));
  }

  const wasi = new WASI({
    version: 'preview1',
    args: [
      argv[0],
      JSON.stringify(metadata),
      join('/sandbox', basename(file))
    ],
    env,
    preopens: preopens,
  });

  const wasm = await WebAssembly.compile(
    await readFile(WASMpath),
  );
  const instance = await WebAssembly.instantiate(wasm, wasi.getImportObject());
  wasi.start(instance);
}