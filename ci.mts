
import { cac } from "cac";
import { readPackage } from 'read-pkg';
import { $ } from 'execa';


const $$ = $({
  stdout: process.stdout,
  stderr: process.stderr,
});


const cli = cac("release");
 

cli.command('publish','publish npm package')
  .option('-t, --tag <tag>', 'Npm tag')
  .option('-p, --push-tags', 'Push tags to github',{default:true})
  .action(async (options: { tag: "canary"|"nightly"|"latest"|"beta"|"alpha", pushTags: boolean }) => {
    const tag = options.tag || 'latest';

    if (tag === "canary") {
      const stdout = await $`git rev-parse --short HEAD`;
      const hash = stdout.stdout?.trim();
    }else if (tag === "nightly") {
      const stdout = await $`git rev-parse --short HEAD`;
      const hash = stdout.stdout?.trim();
    }else if (tag === "beta") {
      const stdout = await $`git rev-parse --short HEAD`;
      const hash = stdout.stdout?.trim();
    }else if (tag === "alpha") {
      const stdout = await $`git rev-parse --short HEAD`;
      const hash = stdout.stdout?.trim();
    } else if (tag === "latest") {
      const stdout = await $`git rev-parse --short HEAD`;
      const hash = stdout.stdout?.trim();
    } else {
      throw new Error(`Invalid tag: ${tag}`);
    }

    const pushTags = options.pushTags || false;
    console.log(tag, pushTags);
    const packageJson = await readPackage();
  });

cli.help();
cli.parse();