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

function checkNodeVersion(version) {
  version = version.split('.');

  if (version[0].replace('v', '') > '19') return true;
  if (version[1] >= '8') return true;

  return false;
}

// If we don't call wasi.start() we get an error, that we can safely (?) ignore in this case.
export async function metaWriter(metadata, file) {
  try {
    await _metaWriter(metadata, file);
  } catch (err) {
    if (err['code'] !== 'ERR_WASI_NOT_STARTED')
      throw err;
  }
}

async function _metaWriter(metadata, file) {
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
    preopens,
  });

  // This lets us use older versions of node, but it's not the best thing to do.
  var importObject = checkNodeVersion(process.version) 
    ? wasi.getImportObject() 
    : { wasi_snapshot_preview1: wasi.wasiImport };

  const wasm = await WebAssembly.compile(
    await readFile(join(
      dirname(import.meta.url.split(':').slice(1).join(':')),
      'meta-writer.wasm'
    )),
  );
  const instance = await WebAssembly.instantiate(wasm, importObject);

  /**
   * wasi.start() throws a segfault when returning or when it tries to access this[kExitCode].
   * 
   * Directly calling the _start module might not be the best thing, but in our case it should be okay.
   * wasi.start() performs some checks to see if parameters, functions and objects 
   * are what they say they are. In our case this should not be necessary. (Meaning that if that were the case,
   * we would have some other problems at the root)
   * 
   * wasi.start() also makes what I think is the mapping between WASM memory and NodeJS memory,
   * but we shouldn't need it.
   */
  instance.exports._start();
}
