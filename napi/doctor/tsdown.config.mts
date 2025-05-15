
import { defineConfig } from 'tsdown'

export default [
  defineConfig({
    entry: '**/src/*.(m)ts',
    outDir: 'es',
    platform: 'node',
    target: 'node22'
  }),
]