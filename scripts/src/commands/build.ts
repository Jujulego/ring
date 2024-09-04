import { inject$ } from '@kyrielle/injector';
import type { CommandModule } from 'yargs';
import { Logger, Spinner } from '../tokens.js';

// Command
const command: CommandModule<unknown, {}> = {
  command: 'build',
  describe: 'Build native code and setup js packages',
  async handler() {
    const logger = inject$(Logger);
    const spinner = inject$(Spinner);

    spinner.start('Building ...');
    logger.info`Hello world!`;
  }
};

export default command;