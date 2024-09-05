import { inject$ } from '@kyrielle/injector';
import fs from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import type { CommandModule } from 'yargs';
import { Logger } from '../tokens.js';
import { exists } from '../utils/fs.js';

// Constants
const ROOT_DIR = path.resolve(import.meta.dirname, '../..');

// Command
const command: CommandModule<unknown> = {
  command: 'link',
  describe: 'Links native executable into js packages',
  async handler() {
    const logger = inject$(Logger);

    // Detect platform
    const platform = process.platform === 'win32' ? 'windows' : process.platform;
    const arch = process.arch === 'x64' ? 'amd64' : process.arch;

    logger.verbose`Detected platform: ${platform}-${arch}`;

    // Search matching package
    const pkgPath = path.join(ROOT_DIR, `npm/ring-${platform}-${arch}/package.json`);
    const pkgExists = await exists(pkgPath);

    if (!pkgExists) {
      logger.warn`Unsupported platform: ${platform}-${arch}`;
      process.exit(1);
    }

    // Link debug executable
    let linkPath = path.join(path.dirname(pkgPath), 'bin/ring');
    let exePath = path.join(ROOT_DIR, 'target/debug/ring-cli');

    if (platform === 'windows') {
      linkPath += '.exe';
      exePath += '.exe';
    }

    if (await exists(linkPath)) {
      logger.info`Already linked`;
    } else {
      logger.debug`Creating symlink ${linkPath}`;
      await fs.symlink(exePath, linkPath, 'file');
      logger.info`Link successfully created`;
    }
  }
};

export default command;