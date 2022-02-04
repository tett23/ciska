import { build } from 'esbuild';
import glob from 'glob';
import process from 'process';

const env = process.env.NODE_ENV ?? 'development';
const entryPoints = glob.sync('./src/**/*.ts');
const watch = process.argv.some((item) => item === '--watch');

await build({
  outdir: `lib/cjs`,
  entryPoints: entryPoints,
  entryNames: '[dir]/[name]',
  platform: 'node',
  format: 'cjs',
  watch,
  sourcemap: true,
  define: {
    'process.env.NODE_ENV': `\"${env}\"`,
  },
});

await build({
  outdir: `lib/esm`,
  entryPoints: entryPoints,
  entryNames: '[dir]/[name]',
  platform: 'node',
  format: 'esm',
  watch,
  sourcemap: true,
  define: {
    'process.env.NODE_ENV': `\"${env}\"`,
  },
});
