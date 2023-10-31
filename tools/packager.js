const toml = require("toml");
const { readFileSync, writeFileSync } = require("node:fs");

var cargo = toml.parse(readFileSync('Cargo.toml', encoding = 'utf-8'));

var package = {
  ...cargo['package'],
  ...{
    author: cargo['package']['authors'][0],
    name: "@orsetto/meta-writer",
    type: "module",
    main: "loader.js",
    dependencies: {
      'suppress-experimental-warnings': "^1.1.17"
    },
    engines: {
      node: ">=14"
    },
    repository: cargo['package']['repository'] === undefined ? undefined : {
      type: "git",
      url: cargo['package']['repository']
    },
    bugs: {
      url: "https://github.com/orsett0/meta-writer/issues"
    },
    private: cargo['package']['publish'] === false ? true : false
  }
}

delete package['publish'];
delete package['authors'];
delete package['edition'];

writeFileSync('pkg/package.json', JSON.stringify(package, undefined, 2));
