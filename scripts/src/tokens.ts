import { token$ } from '@kyrielle/injector';
import { logger$, withTimestamp } from '@kyrielle/logger';
import ora from 'ora';

// Tokens
export const Logger = token$('Logger', () => logger$(withTimestamp()));
export const Spinner = token$('Spinner', () => ora());