import { readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

// 读取主包的 publishConfig
const mainPkg = JSON.parse(readFileSync('package.json', 'utf-8'));
const publishConfig = mainPkg.publishConfig ?? { access: 'public', tag: 'latest' };

console.log(`Patching npm dirs with publishConfig:`, publishConfig);

// 遍历 npm/* 目录
const npmDir = 'npm';
try {
  const dirs = readdirSync(npmDir);

  for (const dir of dirs) {
    const pkgPath = join(npmDir, dir, 'package.json');
    try {
      const pkg = JSON.parse(readFileSync(pkgPath, 'utf-8'));
      pkg.publishConfig = publishConfig;
      writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');
      console.log(`✓ Patched ${dir}`);
    } catch (e) {
      console.warn(`⚠ Skip ${dir}: ${e.message}`);
    }
  }

  console.log('Done!');
} catch (e) {
  console.log('npm/ directory not found, skipping');
}
