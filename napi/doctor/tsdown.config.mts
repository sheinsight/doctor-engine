
import { defineConfig } from 'tsdown'

export default [
  defineConfig({
    entry: '**/src/*.ts',
    outDir: 'esm',
    platform: 'node',
    target: 'node22'
  }),
]