
import { defineConfig } from 'tsdown'

export default [
  defineConfig({
    entry: 'src/**/*.ts',
    outDir:"esm",
    format:"esm",
    platform:"node",
    tsconfig:"tsconfig.json"
  })
]