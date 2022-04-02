import { build } from 'esbuild';
import glob from 'glob';
import process from 'process';
import fs from 'fs/promises';

const watch = process.argv.some((item) => item === '--watch');

await build({
  outdir: `lib/useCases/cjs`,
  entryPoints: glob.sync('./src/useCases/**/*.ts'),
  entryNames: '[dir]/[name]',
  platform: 'node',
  format: 'cjs',
  watch,
  sourcemap: true,
});

await build({
  outdir: `lib/useCases/esm`,
  entryPoints: glob.sync('./src/useCases/**/*.ts'),
  entryNames: '[dir]/[name]',
  platform: 'node',
  format: 'esm',
  watch,
  sourcemap: true,
});

await build({
  outdir: `lib/client/cjs`,
  entryPoints: glob.sync('./src/client/**/*.ts'),
  entryNames: '[dir]/[name]',
  platform: 'node',
  format: 'cjs',
  watch,
  sourcemap: true,
});

await build({
  outdir: `lib/client/esm`,
  entryPoints: glob.sync('./src/client/**/*.ts'),
  entryNames: '[dir]/[name]',
  platform: 'node',
  format: 'esm',
  watch,
  sourcemap: true,
});

await fs.copyFile('./useCases.d.ts', './lib/useCases/index.d.ts');
await fs.copyFile('./client.d.ts', './lib/client/index.d.ts');
