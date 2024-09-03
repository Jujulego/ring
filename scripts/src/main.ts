import process from 'node:process';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import { loggerMiddleware } from './middlewares/logger.middleware.js';

// Bootstrap
const parser = yargs(hideBin(process.argv))
  .scriptName('ring-dt');

loggerMiddleware(parser);

parser
  .demandCommand()
  .recommendCommands()
  .strictCommands();

await parser.parseAsync();