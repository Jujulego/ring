#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const version = process.argv[2].replace(/^v/, '');
const packages = ['ring'];

while (packages.length) {
  const name = packages.pop();

  const manifestPath = path.resolve(__dirname, '..', 'npm', name, 'package.json');
  const manifest = require(manifestPath);

  manifest.version = version;

  if (manifest.optionalDependencies) {
    for (const dep in manifest.optionalDependencies) {
      manifest.optionalDependencies[dep] = version;
      packages.push(dep.replace(/^@jujulego\//, ''));
    }
  }

  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2));
}