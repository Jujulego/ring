import globals from 'globals';
import js from '@eslint/js';

export default [
  {
    ignores: ['.pnp.*', '.yarn']
  },
  {
    languageOptions: { globals: globals.node },
    linterOptions: {
      reportUnusedDisableDirectives: 'error'
    }
  },
  {
    files: ['**/*.js', '**/*.jsx', 'npm/ring/bin/ring'],
    rules: {
      ...js.configs.recommended.rules,
      quotes: ['error', 'single'],
      semi: ['error', 'always'],
    }
  },
];