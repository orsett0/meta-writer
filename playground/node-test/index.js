'use strict';
const { argv } = require('node:process');

var datetime = new Date();

(async () => {
  const lofty = await import('meta-writer');

  await lofty.meta_writer(
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
