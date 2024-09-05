import process from 'node:process';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import link from './commands/link.js';
import { loggerMiddleware } from './middlewares/logger.middleware.js';

// Bootstrap
const parser = yargs(hideBin(process.argv))
  .scriptName('ring-dt');

loggerMiddleware(parser);

parser
  .command(link)
  .demandCommand()
  .recommendCommands()
  .strictCommands();

await parser.parseAsync();