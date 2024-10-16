import { defineQuickFormat, q$, qarg, qerror, qprop, qwrap } from '@jujulego/quick-tag';
import { globalScope$, inject$ } from '@kyrielle/injector';
import {
  type Log,
  type LogDelay,
  logDelay$,
  LogLevel,
  type LogLevelKey,
  qLogDelay,
  type WithDelay,
} from '@kyrielle/logger';
import type { ColorName, ModifierName } from 'chalk';
import { chalkTemplateStderr } from 'chalk-template';
import { filter$, flow$, observer$ } from 'kyrielle';
import os from 'node:os';
import process from 'node:process';
import type { Argv } from 'yargs';
import { Logger, Spinner } from '../tokens.js';

// Constants
const VERBOSITY_LEVEL: Record<number, LogLevelKey> = {
  1: 'verbose',
  2: 'debug',
};

const LEVEL_COLORS = {
  [LogLevel.debug]: 'grey',
  [LogLevel.verbose]: 'blue',
  [LogLevel.info]: 'reset',
  [LogLevel.warning]: 'yellow',
  [LogLevel.error]: 'red',
} satisfies Record<LogLevel, ColorName | ModifierName>;

// Format
const logColor = defineQuickFormat((level: LogLevel) => LEVEL_COLORS[level])(qprop<Log, 'level'>('level'));
const logFormat = qwrap(chalkTemplateStderr)
  .fun`#?:${qprop('label')}{grey [${q$}]} ?#{${logColor} ${qprop('message')} {grey +${qLogDelay(qarg<WithDelay>())}}#?:${qerror(qprop<Log>('error'))}${os.EOL}${q$}?#}`;

// Middleware
export function loggerMiddleware(parser: Argv) {
  return parser
    .option('verbose', {
      alias: 'v',
      type: 'count',
      description: 'Set verbosity level',
      coerce: (cnt: number) => VERBOSITY_LEVEL[Math.min(cnt, 2)]
    })
    .middleware(({ verbose }) => {
      const logLevel = verbose ? LogLevel[verbose] : LogLevel.info;

      flow$(
        inject$(Logger),
        filter$((log) => log.level >= logLevel),
        logDelay$(),
        observer$({
          next(log: Log & LogDelay) {
            const spinner = globalScope$().get(Spinner);

            spinner?.clear();
            process.stderr.write(logFormat(log) + os.EOL);
          }
        })
      );
    });
}