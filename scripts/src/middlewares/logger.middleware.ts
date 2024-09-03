import {
  type Log, logDebugFilter$, logDelay$,
  LogLevel,
  type LogLevelKey,
  qLogDelay, toStderr,
  type WithDelay,
} from '@kyrielle/logger';
import { defineQuickFormat, q$, qarg, qerror, qprop, qwrap } from '@jujulego/quick-tag';
import { chalkTemplateStderr } from 'chalk-template';
import os from 'node:os';
import type { ColorName, ModifierName } from 'chalk';
import type { Argv } from 'yargs';
import { filter$, flow$ } from 'kyrielle';
import { inject$ } from '@kyrielle/injector';
import { Logger } from '../tokens.js';

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
        logDebugFilter$(),
        logDelay$(),
        toStderr(logFormat)
      );
    });
}