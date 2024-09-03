import { token$ } from '@kyrielle/injector';
import { logger$, withTimestamp } from '@kyrielle/logger';

// Tokens
export const Logger = token$('Logger', () => logger$(withTimestamp()));