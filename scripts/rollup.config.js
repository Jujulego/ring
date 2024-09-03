import { swc } from '@jujulego/vite-plugin-swc';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import fs from 'node:fs';

const pkg = JSON.parse(fs.readFileSync('./package.json', 'utf-8'));

/** @type {import('rollup').RollupOptions} */
const options = {
  input: {
    main: './src/main.ts'
  },
  output: {
    dir: 'dist',
    format: 'esm',
    sourcemap: true,
    chunkFileNames: '[name].js',
    generatedCode: 'es5'
  },
  plugins: [
    nodeResolve({ exportConditions: ['node'] }),
    swc()
  ],
  external: [
    ...Object.keys(pkg.dependencies),
  ]
};

export default options;