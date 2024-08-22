#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const template = require('./templates/package.tpl.json');

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

for (const { platform, arch } of targets) {
  template.name = `@jujulego/ring-${platform}-${arch}`;
  template.description = `Ring ${platform} ${arch} binary.`;
  template.os = [nodeOsLookup[platform]];
  template.cpu = [nodeArchLookup[arch]];
  //template.version = version;

  const outputPath = path.resolve(__dirname, '..', 'npm', `ring-${platform}-${arch}`);
  fs.rmSync(outputPath, { recursive: true, force: true });
  fs.mkdirSync(path.join(outputPath, 'bin'), { recursive: true });

  if (platform === 'windows') {
    fs.copyFileSync(
        path.join(__dirname, 'templates', 'bin', 'ring'),
        path.join(outputPath, 'bin', 'ring')
    );
  }

  fs.writeFileSync(
      path.join(outputPath, 'package.json'),
      JSON.stringify(template, null, 2)
  );
}