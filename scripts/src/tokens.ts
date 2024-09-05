import { token$ } from '@kyrielle/injector';
import { logger$, withTimestamp } from '@kyrielle/logger';
import yoctoSpinner from 'yocto-spinner';

// Tokens
export const Logger = token$('Logger', () => logger$(withTimestamp()));
export const Spinner = token$('Spinner', () => yoctoSpinner());