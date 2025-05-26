
import { cac } from "cac";
import { initializeLogger, Standards } from "@shined/doctor-binding"
import { performance } from "node:perf_hooks"
const cli = cac("doctor");


cli.command('','check project health')
  .option('-v, --verbose', 'Verbose output')
  .option('--cwd <path>', 'Current working directory')
  .action(async (options) => {

    const start = performance.now();

    initializeLogger();

    const cwd = options.cwd || process.cwd();

    const standards = await Standards.create(cwd);

    const res = await standards.validateAll();

    if (res.length > 0) {
      process.exit(1);
    }

    const end = performance.now();
    console.log(`Time taken: ${end - start} milliseconds`);

});

cli.help();

cli.parse();

