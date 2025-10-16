
import { cac } from "cac";
import { initializeLogger, JsSpecifications } from "@shined/doctor-binding"
import { performance } from "node:perf_hooks"
const cli = cac("doctor");




cli.command('','check project health')
  .option('-v, --verbose', 'Verbose output')
  .option('--cwd <path>', 'Current working directory')
  .action(async (options) => {

    const start = performance.now();

    initializeLogger();

    const cwd = options.cwd || process.cwd();

    const standards = await JsSpecifications.create(cwd,{
      quiet: false,
      withDashboard:true,
    });

    const res = await standards.validateAll();

    const errorCount = res.reduce((count, msg) => 
      count + msg.diagnostics.filter(d => d.severity === "Error").length, 0
    );

    if (errorCount > 0) {
      console.log(`Found ${errorCount} errors`);
      process.exit(1);
    }

    const end = performance.now();
    console.log(`Time taken: ${end - start} milliseconds`);

});

cli.command('fix','fix lint')
  .option('-v, --verbose', 'Verbose output')
  .option('--cwd <path>', 'Current working directory')
  .action(async (options) => {
    const start = performance.now();
    const cwd = options.cwd || process.cwd();
    const standards = await JsSpecifications.create(cwd);
    const res = await standards.fixLint();

    const errorCount = res.reduce((count, msg) =>
      count + msg.diagnostics.filter(d => d.severity === "Error").length, 0
    );
    if (errorCount > 0) {
      console.log(`Found ${errorCount} errors`);
      process.exit(1);
    }
    const end = performance.now();
    console.log(`Time taken: ${end - start} milliseconds`);
  });

cli.help();

cli.parse();

