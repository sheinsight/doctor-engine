#!/usr/bin/env node

import { cac } from "cac";
import { innerLint,NaPiCategory,initializeLogger,doctor } from "../index.js"
import { performance } from "node:perf_hooks"
const cli = cac("doctor");



// cli.command("lint", "Lint the project").alias('').action(async () => {
//   console.log("lint");
  
//   const res = await innerLint({
//     cwd: process.cwd(),
//     verbose: true,
//   }, NaPiCategory.V20250601Inner)

// });

cli.command('','check project health')
  .option('-v, --verbose', 'Verbose output')
  .option('--cwd <path>', 'Current working directory')
  .action(async (options) => {

    const start = performance.now();

    initializeLogger();

    const res = await doctor(options.cwd || process.cwd(),{
      // verbose: true,
    });

    const end = performance.now();
    console.log(`Time taken: ${end - start} milliseconds`);

});





cli.help();


cli.parse();

