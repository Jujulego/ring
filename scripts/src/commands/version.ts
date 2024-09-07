import { inject$ } from '@kyrielle/injector';
import fs from 'node:fs/promises';
import path from 'node:path';
import type { CommandModule } from 'yargs';
import { Logger } from '../tokens.js';

// Constants
const ROOT_DIR = path.resolve(import.meta.dirname, '../..');

// Types
export interface VersionArgs {
  value: string;
}

interface PackageManifest {
  version: string;
  optionalDependencies?: Record<string, string>;
}

// Command
const command: CommandModule<unknown, VersionArgs> = {
  command: 'set version <value>',
  describe: 'Sets all ring js packages to given version',
  builder: (yargs) => yargs
    .positional('value', { type: 'string' })
    .demandOption(['value']),
  async handler(args: VersionArgs) {
    const logger = inject$(Logger);

    // Remove "v" prefix
    if (args.value.startsWith('v')) {
      args.value = args.value.substring(1);
    }

    // Detect and update manifests
    const packages = ['ring'];

    while (packages.length > 0) {
      const pkg = packages.pop()!;
      const manifestPath = path.join(ROOT_DIR, `npm/${pkg}/package.json`);

      logger.verbose(`Updating ${manifestPath}`);

      const manifest = JSON.parse(await fs.readFile(manifestPath, 'utf8')) as PackageManifest;
      manifest.version = args.value;

      if (manifest.optionalDependencies) {
        for (const dep in manifest.optionalDependencies) {
          manifest.optionalDependencies[dep] = args.value;
          packages.push(dep.replace(/^@jujulego\//, ''));
        }
      }

      await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2));
    }
  }
};

export default command;