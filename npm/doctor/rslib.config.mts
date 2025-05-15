import { defineConfig } from '@rslib/core';

export default defineConfig({
  lib: [
    {
      bundle:false,
      format: 'esm',
      syntax: 'es2021',
      output: {
        target: 'node',
      },
    },
  ], 
});