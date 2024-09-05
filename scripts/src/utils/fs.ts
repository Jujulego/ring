import { inject$ } from '@kyrielle/injector';
import fs from 'node:fs/promises';
import { Logger } from '../tokens.js';

export async function exists(path: string): Promise<boolean> {
  const logger = inject$(Logger);
  logger.debug`Testing existence of ${path}`;

  try {
    await fs.access(path, fs.constants.F_OK);
    return true;
  } catch (err) {
    if ((err as NodeJS.ErrnoException).code === 'ENOENT') {
      return false;
    }

    throw err;
  }
}