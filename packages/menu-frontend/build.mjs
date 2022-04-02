import { build } from 'esbuild';
import glob from 'glob';
import process from 'process';
import fs from 'fs/promises';

const entryPoints = glob.sync('./src/**/*.ts');
const watch = process.argv.some((item) => item === '--watch');

await build({
  outdir: `lib/cjs`,
  entryPoints: entryPoints,
  entryNames: '[dir]/[name]',
  platform: 'browser',
  format: 'cjs',
  bundle: true,
  watch,
  sourcemap: true,
});

await build({
  outdir: `lib/esm`,
  entryPoints: entryPoints,
  entryNames: '[dir]/[name]',
  platform: 'browser',
  format: 'esm',
  watch,
  sourcemap: true,
});

await fs.copyFile('./src/index.html', './lib/cjs/index.html');
await fs.copyFile('./src/index.html', './lib/esm/index.html');
