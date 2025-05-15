import { defineConfig } from '@rslib/core';

export default defineConfig({
  lib: [
    {
      bundle:false,
      format: 'esm',
      syntax: 'es2021',
      // dts:{
      //   bundle:false,
      //   distPath:"es",
      //   autoExtension:true
      // },
      dts: true,
      // outBase:"es",
      output: {
        target: 'node',
        distPath:{
          root:"es",
          js: './',
        }
      },
    },
  ], 
});