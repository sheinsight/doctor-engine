import { cac } from "cac";
import { innerLint,NaPiCategory,initializeLogger } from "../index.js"

const cli = cac("doctor");

initializeLogger();

cli.command("lint", "Lint the project").action(async () => {
  const res = await innerLint({
    cwd: process.cwd(),
    verbose: true,
  }, NaPiCategory.V20250601Inner)

});


cli.parse();

