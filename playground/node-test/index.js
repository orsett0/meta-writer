'use strict';
import { argv } from 'node:process';

import metaWriter from '@orsetto/meta-writer';

import AsyncQueue from './async-queue.js';

(async () => {
  for (var max = 2; max < 1000; max += 1) {
    var lst = [
      new AsyncQueue(
        'eheh',
        1,
        async ({index, date}) => {
          process.stdout.write('\rmax:' + max.toString());
          await metaWriter(
            {
              'TrackTitle': date.toString(),
              'TrackArtist': date.toString(),
              'AlbumTitle': date.toString(),
              'Genre': date.toString(),
              'rDNS': [
                { 'mean': 'com.apple.iTunes', 'name': 'MEDIA', 'data': date.toString() }
              ],
              'apID': date.toString(),
              'TrackNumber': date.getHours(),
              'TrackTotal': date.getMinutes(),
              'FrontCover': 'samples/cover.jpg'
            },
            argv[2]
          );
        }
      )
    ];

    for (var i = 1; i < max; i++) {
      lst.push(new AsyncQueue(
        i.toString(),
        1,
        async ({index, date}) => {
          index = index - 1;
          await lst.at(index).push({index, date});
        }
      ));
    }

    var index = lst.length - 1;
    var date = new Date();

    await lst.at(index).push({index, date});
  }
})();
