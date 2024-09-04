import js from '@eslint/js';
import globals from 'globals';
import ts from 'typescript-eslint';

export default ts.config(
  {
    ignores: ['.pnp.*', '.yarn', 'dist', 'coverage']
  },
  {
    languageOptions: { globals: globals.node },
    linterOptions: {
      reportUnusedDisableDirectives: 'error'
    }
  },
  {
    files: ['**/*.js', '**/*.jsx', '**/*.ts', '**/*.tsx', 'npm/ring/bin/ring'],
    extends: [
      js.configs.recommended
    ],
    rules: {
      quotes: ['error', 'single'],
      semi: ['error', 'always'],
    }
  },
  {
    files: ['**/*.ts', '**/*.tsx'],
    extends: [...ts.configs.recommendedTypeChecked],
    languageOptions: {
      parserOptions: {
        project: ['./scripts/tsconfig.json'],
        tsconfigRootDir: import.meta.dirname,
      }
    },
    rules: {
      '@typescript-eslint/no-unused-expressions': ['error', {
        allowTaggedTemplates: true
      }]
    }
  }
);