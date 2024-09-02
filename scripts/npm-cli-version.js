#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';

const version = process.argv[2]?.replace(/^v/, '') ?? '0.0.0-dev';
const packages = ['ring'];

while (packages.length) {
  const name = packages.pop();

  const manifestPath = path.resolve(import.meta.dirname, '..', 'npm', name, 'package.json');
  const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf-8'));

  manifest.version = version;

  if (manifest.optionalDependencies) {
    for (const dep in manifest.optionalDependencies) {
      manifest.optionalDependencies[dep] = version;
      packages.push(dep.replace(/^@jujulego\//, ''));
    }
  }

  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2));
}