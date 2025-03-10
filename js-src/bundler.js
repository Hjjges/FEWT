const esbuild = require('esbuild');

esbuild.build({
    entryPoints: ['./terminal.js'],
    bundle: true,
    minify: true,
    outfile: '../assets/bundled.js',
    loader: { '.css': 'text' },
}).catch(() => process.exit(1));