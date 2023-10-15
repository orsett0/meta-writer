'use strict';
import { argv } from 'node:process';
import meta_writer from '@orsetto/meta-writer';

var datetime = new Date();

(async () => {

  await meta_writer(
    {
      'TrackTitle': datetime.toString(),
      'TrackArtist': datetime.toString(),
      'AlbumTitle': datetime.toString(),
      'Genre': datetime.toString(),
      'rDNS': [
        {'mean': 'com.apple.iTunes', 'name': 'MEDIA', 'data': datetime.toString() }
      ],
      'apID': datetime.toString(),
      'TrackNumber': datetime.getHours(),
      'TrackTotal': datetime.getMinutes(),
      'FrontCover': 'samples/cover.jpg'
    },
    argv[2]
  );
})();
