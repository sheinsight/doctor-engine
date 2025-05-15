import { cac } from "cac";
import { doctor, initializeLogger } from "@shined/doctor-binding";
import { performance } from "node:perf_hooks";

//#region src/cli.mts
const cli = cac("doctor");
cli.command("", "check project health").option("-v, --verbose", "Verbose output").option("--cwd <path>", "Current working directory").action(async (options) => {
	const start = performance.now();
	initializeLogger();
	const cwd = options.cwd || process.cwd();
	const res = await doctor(cwd);
	if (res.length > 0) process.exit(1);
	const end = performance.now();
	console.log(`Time taken: ${end - start} milliseconds`);
});
cli.help();
cli.parse();

//#endregion