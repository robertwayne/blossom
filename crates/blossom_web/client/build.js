const esbuild = require("esbuild")
const autoprefixer = require("autoprefixer")
const cssnano = require("cssnano")

console.log("Minifying and bundling assets...")

esbuild
    .build({
        entryPoints: ["src/index.ts"],
        bundle: true,
        minify: true,
        outfile: "../dist/index.js",
    })
    .catch((e) => console.error(e.error))

console.log('Created "index.js" and "index.css" in "dist" directory.')
