import * as __WEBPACK_EXTERNAL_MODULE_cac__ from "cac";
import * as __WEBPACK_EXTERNAL_MODULE__shined_doctor_binding_71127777__ from "@shined/doctor-binding";
import * as __WEBPACK_EXTERNAL_MODULE_node_perf_hooks_81520749__ from "node:perf_hooks";
const cli = (0, __WEBPACK_EXTERNAL_MODULE_cac__.cac)("doctor");
cli.command('', 'check project health').option('-v, --verbose', 'Verbose output').option('--cwd <path>', 'Current working directory').action(async (options)=>{
    const start = __WEBPACK_EXTERNAL_MODULE_node_perf_hooks_81520749__.performance.now();
    (0, __WEBPACK_EXTERNAL_MODULE__shined_doctor_binding_71127777__.initializeLogger)();
    const cwd = options.cwd || process.cwd();
    const res = await (0, __WEBPACK_EXTERNAL_MODULE__shined_doctor_binding_71127777__.doctor)(cwd);
    if (res.length > 0) process.exit(1);
    const end = __WEBPACK_EXTERNAL_MODULE_node_perf_hooks_81520749__.performance.now();
    console.log(`Time taken: ${end - start} milliseconds`);
});
cli.help();
cli.parse();
