#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';

// Map to node os and arch names.
const targets = [
  { platform: 'linux', arch: 'amd64' },
  { platform: 'windows', arch: 'amd64' },
  { platform: 'macos', arch: 'amd64' },
  { platform: 'macos', arch: 'arm64' },
];

const nodeOsLookup = {
  linux: 'linux',
  windows: 'win32',
  macos: 'darwin',
};

const nodeArchLookup = {
  amd64: 'x64',
  arm64: 'arm64',
};

const humanizedArchLookup = {
  amd64: 'x64',
  arm64: 'arm64',
};

const template = JSON.parse(fs.readFileSync(path.resolve(import.meta.dirname, 'templates', 'package.tpl.json'), 'utf-8'));

for (const { platform, arch } of targets) {
  template.name = `@jujulego/ring-${platform}-${arch}`;
  template.description = `Ring ${platform} ${humanizedArchLookup[arch]} binary.`;
  template.os = [nodeOsLookup[platform]];
  template.cpu = [nodeArchLookup[arch]];

  const outputPath = path.resolve(import.meta.dirname, '..', 'npm', `ring-${platform}-${arch}`);
  fs.rmSync(outputPath, { recursive: true, force: true });
  fs.mkdirSync(path.join(outputPath, 'bin'), { recursive: true });

  if (platform === 'windows') {
    fs.copyFileSync(
        path.join(import.meta.dirname, 'templates', 'bin', 'ring'),
        path.join(outputPath, 'bin', 'ring')
    );
  }

  fs.writeFileSync(
      path.join(outputPath, 'package.json'),
      JSON.stringify(template, null, 2)
  );
}